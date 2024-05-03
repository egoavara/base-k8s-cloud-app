use anyhow::anyhow;
use sqlx::{Executor, PgPool};
use std::collections::HashMap;
use temporal_sdk::{ActContext, ActExitValue, LocalActivityOptions, WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::coresdk::activity_result::activity_resolution::Status;
use temporal_sdk_core::protos::coresdk::activity_result::Success;
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;
use temporal_sdk_core::protos::temporal::api::common::v1::Payload;
use tokio::select;
use tracing::{info, info_span, instrument, Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

use crate::agent_types::{Agent, CreateAgent};
use crate::utils::{parse_any_payload, parse_any_traceparent, parse_at_payload, wait_json_success, AsJsonPayloadTraceExt};

pub async fn create_agent(ctx: WfContext) -> WorkflowResult<Agent> {
    // args
    let trace = parse_any_traceparent(&ctx.get_args()).unwrap_or(Span::current().context());
    let arg = parse_at_payload::<CreateAgent>(0, &ctx.get_args())?;
    // tracing
    let span = info_span!("create_agent", agent_id = tracing::field::Empty, name = &arg.name);
    span.set_parent(trace);
    // flow control
    let cancel = ctx.cancelled();
    // running
    let run = async {
        let span = Span::current();
        let trace = span.context();
        info!("start create_agent");
        ctx.upsert_memo(vec![(
            "agent".to_string(),
            Payload {
                metadata: HashMap::new(),
                data: serde_json::to_vec(&arg).unwrap(),
            },
        )]);
        let CreateAgent { name, runtime } = arg;
        let agent_id = wait_json_success!(Uuid : ctx
            .local_activity(LocalActivityOptions {
                activity_type: "generate_uuid_v4".to_string(),
                input: ().propagate_json(&trace).unwrap(),
                ..Default::default()
            })
        );
        let result = Agent { agent_id, name, runtime };
        // information
        ctx.upsert_search_attributes(vec![("GeneratedId".to_string(), agent_id.as_json_payload()?)]);
        ctx.upsert_memo(vec![("agent".to_string(), result.as_json_payload()?)]);
        // save
        wait_json_success!(() : ctx
            .local_activity(LocalActivityOptions {
                activity_type: "save_agent".to_string(),
                input: result.clone().propagate_json(&trace).unwrap(),
                ..Default::default()
            })
        );
        // return
        Ok(WfExitValue::Normal(result))
    }
    .instrument(span);
    select! {
        _ = cancel => {
            return Ok(WfExitValue::Cancelled);
        }
        result = run => {
            return result
        }
    }
}

#[instrument(level = "info", skip(ctx))]
pub async fn save_agent(ctx: ActContext, arg: Agent) -> Result<ActExitValue<()>, anyhow::Error> {
    let pool = ctx.app_data::<PgPool>().unwrap();
    sqlx::query!(r#"INSERT INTO agent (agent_id, name, runtime) VALUES ($1, $2, $3)"#, arg.agent_id, arg.name, serde_json::to_value(arg.runtime).unwrap())
        .execute(pool)
        .await?;
    Ok(ActExitValue::Normal(()))
}

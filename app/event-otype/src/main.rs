use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;
use futures::{FutureExt, TryFutureExt};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use temporal_client::ClientOptionsBuilder;
use temporal_client::tonic::codegen::Body;
use temporal_sdk::{ActContext, ActExitValue, LocalActivityOptions, WfContext, WfExitValue, Worker, WorkflowFunction, WorkflowResult};
use temporal_sdk_core::{CoreRuntime, WorkerConfigBuilder};
use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::protos::coresdk::activity_result::activity_resolution::Status;
use temporal_sdk_core::protos::coresdk::activity_result::Success;
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct CreateServer {
    name: String,
    ip: Vec<String>,
    fqdn: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Server {
    server_id: uuid::Uuid,
    name: String,
    ip: Vec<String>,
    fqdn: String,
}

pub struct Runtime {
    inner: Arc<RuntimeInner>,
}

pub struct RuntimeInner {
    pg: PgPool,
}

impl Runtime {
    pub fn create_server(&self) -> WorkflowFunction {
        let inner = self.inner.clone();
        let handler = move |ctx: WfContext| inner.create_server(ctx);
        WorkflowFunction::new(handler)
    }
}

async fn create_server(ctx: WfContext) -> WorkflowResult<Server> {
    println!("===================================");
    println!("create_server");
    if ctx.get_args().len() < 1 {
        return Err(anyhow::anyhow!("missing arguments"));
    }
    let create_server: CreateServer = serde_json::from_slice(&ctx.get_args()[0].data)?;
    println!("create_server : {:?}", &create_server);
    let server_id = ctx
        .local_activity(LocalActivityOptions {
            activity_type: "create_server_id".to_string(),
            input: ().as_json_payload().unwrap(),
            ..Default::default()
        })
        .map(|x| {
            println!("local_activity : {:?}", x);
            match x.status {
                Some(Status::Completed(Success { result: Some(result) })) => serde_json::from_slice::<uuid::Uuid>(result.data.as_slice()).map_err(|e| anyhow::anyhow!("failed to deserialize server_id : {:?}", e)),
                Some(Status::Completed(Success { result: None })) => Err(anyhow!("missing result")),
                None => Err(anyhow!("missing status")),
                Some(Status::Backoff(backoff)) => Err(anyhow!("backoff")),
                Some(Status::Cancelled(canceled)) => Err(anyhow!("canceled")),
                Some(Status::Failed(failed)) => Err(anyhow!("failed")),
            }
        })
        .map_err(|e| {
            println!("error : {:?}", e);
            e
        })
        .await?;
    println!("server_id : {:?}", server_id);
    ctx.upsert_search_attributes(vec![("GeneratedId".to_string(), server_id.to_string().as_json_payload().unwrap())]);
    println!("upsert_search_attributes");
    let timer = ctx.timer(Duration::new(4, 0));
    timer.await;

    println!("timer");

    let mut server = Server {
        server_id,
        name: create_server.name,
        ip: create_server.ip,
        fqdn: create_server.fqdn,
    };
    println!("server : {:?}", &server);
    Ok(WfExitValue::Normal(server))
}
async fn create_server_id<'a>(act: ActContext, arg: ()) -> Result<ActExitValue<Uuid>, anyhow::Error> {
    println!("create_server_id");
    // Ok(uuid::Uuid::new_v4().to_string())
    Ok(ActExitValue::Normal(uuid::Uuid::new_v4()))
}
// async fn create_server_db_commit
// async fn create_server_cache_commit
// async fn create_server_cache_rollback
//
// async fn create_server_dns_bind
// async fn create_server_dns_check
// async fn create_server_dns_unbind
//
// async fn create_server_admin_check

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .max_lifetime(Some(Duration::from_secs(300)))
        .connect_with(
            PgConnectOptions::new()
                .host("postgresql-hl.persistence-system.svc")
                .port(5432)
                .username("postgres-user")
                .password("911jnyyWPc7ZdEjsO5JpfP6GBb-PFslsMwbUCUDD8j7EUxYifZVm3l7MXVjOoQlF")
                .database("public"),
        )
        .await
        .unwrap();

    let temporal_url = temporal_sdk_core::Url::parse("http://temporal-frontend-headless.event-system.svc:7233").unwrap();

    let runtime_option = TelemetryOptionsBuilder::default().attach_service_name(false).metric_prefix("event-otype".to_string()).build().unwrap();
    let runtime = CoreRuntime::new_assume_tokio(runtime_option).unwrap();
    let client_option = ClientOptionsBuilder::default()
        .identity("event-otype".to_string())
        .target_url(temporal_url)
        .client_name("event-otype".to_string())
        .client_version("0.1.0".to_string())
        .build()
        .unwrap();
    let mut client = Arc::new(client_option.connect("default", None).await.unwrap());

    let worker_config = WorkerConfigBuilder::default().namespace("default").worker_build_id("event-otype-0".to_string()).task_queue("default-queue").build().unwrap();
    let core_worker = Arc::new(temporal_sdk_core::init_worker(&runtime, worker_config, client.clone()).unwrap());

    let mut worker = Worker::new_from_core(core_worker, "test?".to_string());
    worker.insert_app_data(pool);
    worker.register_wf("create-event", create_server);
    worker.register_activity("create_server_id", create_server_id);
    // worker.set_worker_interceptor()
    // worker.register_activity("create_server_db_commit", create_server_id);
    worker.run().await.unwrap();
}

// temporal workflow start --type create-event --task-queue default-queue --input '{"name": "testserver", "ip": ["1.2.3.4"], "fqdn": "testserver.example.com"}'

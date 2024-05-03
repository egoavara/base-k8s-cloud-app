use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;

use anyhow::anyhow;
use futures::future::BoxFuture;
use futures::FutureExt;
use opentelemetry::Context;
use opentelemetry_http::HeaderInjector;
use serde::{Deserialize, Serialize};
use temporal_sdk::{ActContext, ActExitValue, IntoActivityFunc};
use temporal_sdk_core::protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};
use temporal_sdk_core::protos::temporal::api::common::v1::Payload;
use temporal_sdk_core::protos::{ENCODING_PAYLOAD_KEY, JSON_ENCODING_VAL};
use tracing::{info, info_span, instrument, Instrument, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use uuid::Uuid;

pub use wf_context_ext::*;

mod wf_context_ext;

pub fn parse_any_traceparent(payloads: &[Payload]) -> Option<Context> {
    payloads.iter().filter_map(|x| parse_traceparent(x)).next()
}

pub fn parse_traceparent(payload: &Payload) -> Option<Context> {
    if payload.metadata.contains_key("traceparent") {
        let header = payload.metadata.iter().filter_map(|(k, v)| String::from_utf8(v.clone()).map(|v| (k.clone(), v)).ok()).collect::<HashMap<String, String>>();
        return Some(opentelemetry::global::get_text_map_propagator(|propagator| propagator.extract(&header)));
    }

    if !payload.is_json_payload() {
        return None;
    }
    match serde_json::from_slice::<HashMap<String, String>>(payload.data.as_slice()) {
        | Ok(map) => {
            if !map.contains_key("traceparent") {
                return None;
            }
            return Some(opentelemetry::global::get_text_map_propagator(|propagator| propagator.extract(&map)));
        }
        | Err(e) => return None,
    }
}

pub fn parse_any_payload<'de, T: Deserialize<'de>>(parent: &'de [Payload]) -> Option<T> {
    parent.iter().filter_map(|x| parse_payload(x).ok()).next()
}

pub fn parse_at_payload<'de, T: Deserialize<'de>>(index: usize, parent: &'de [Payload]) -> Result<T, anyhow::Error> {
    if index >= parent.len() || index < 0 {
        return Err(anyhow!("index out of range"));
    }
    parse_payload(&parent[index])
}
pub fn parse_payload<'de, T: Deserialize<'de>>(parent: &'de Payload) -> Result<T, anyhow::Error> {
    if !parent.is_json_payload() {
        return Err(anyhow!("not json payload"));
    }
    let data = serde_json::from_slice(parent.data.as_slice());
    data.map_err(|e| anyhow!("failed to deserialize payload : {:?}", e))
}

#[instrument(level = "info", skip_all, fields(result))]
pub async fn generate_uuid_v4(act: ActContext, arg: ()) -> Result<ActExitValue<Uuid>, anyhow::Error> {
    let result = uuid::Uuid::new_v4();
    Span::current().record("result", result.to_string());
    Ok(ActExitValue::Normal(result))
}

pub struct TracingActivity<A, Rf> {
    func: Box<dyn (Fn(ActContext, A) -> Rf) + Sync + Send + 'static>,
}

impl<A, Rf> TracingActivity<A, Rf> {
    pub fn new<F>(func: F) -> Self
    where
        F: (Fn(ActContext, A) -> Rf) + Sync + Send + 'static,
    {
        Self { func: Box::new(func) }
    }
}

impl<A, Rf, R, O> IntoActivityFunc<A, R, O> for TracingActivity<A, Rf>
where
    Rf: Future<Output = Result<R, anyhow::Error>> + Send + 'static,
    R: Into<ActExitValue<O>>,
    O: AsJsonPayloadExt,
    A: FromJsonPayloadExt + Send + 'static,
{
    fn into_activity_fn(self) -> Arc<dyn Fn(ActContext, Payload) -> BoxFuture<'static, Result<ActExitValue<Payload>, anyhow::Error>> + Send + Sync> {
        let wrapper = move |ctx: ActContext, input: Payload| {
            let trace = parse_traceparent(&input);
            let func = &self.func;
            let span = if let Some(trace) = trace {
                let span = info_span!("activity");
                span.set_parent(trace);
                span
            } else {
                info_span!("activity")
            };

            // Some minor gymnastics are required to avoid needing to clone the function
            match A::from_json_payload(&input) {
                | Ok(deser) => func(ctx, deser)
                    .instrument(span)
                    .map(|r| {
                        r.and_then(|r| {
                            let exit_val: ActExitValue<O> = r.into();
                            Ok(match exit_val {
                                | ActExitValue::WillCompleteAsync => ActExitValue::WillCompleteAsync,
                                | ActExitValue::Normal(x) => ActExitValue::Normal(x.as_json_payload()?),
                            })
                        })
                    })
                    .boxed(),
                | Err(e) => async move { Err(e.into()) }.instrument(span).boxed(),
            }
        };
        Arc::new(wrapper)
    }
}

pub trait AsJsonPayloadTraceExt {
    fn propagate_json(&self, context: &Context) -> anyhow::Result<Payload>;
}

impl<T> AsJsonPayloadTraceExt for T
where
    T: Serialize,
{
    fn propagate_json(&self, context: &Context) -> anyhow::Result<Payload> {
        let as_json = serde_json::to_string(self)?;
        let mut metadata = HashMap::new();
        metadata.insert(ENCODING_PAYLOAD_KEY.to_string(), JSON_ENCODING_VAL.as_bytes().to_vec());

        let mut temp = HashMap::<String, String>::new();
        opentelemetry::global::get_text_map_propagator(|propagator| propagator.inject_context(context, &mut temp));
        for (k, v) in temp {
            metadata.insert(k, v.as_bytes().to_vec());
        }

        Ok(Payload { metadata, data: as_json.into_bytes() })
    }
}

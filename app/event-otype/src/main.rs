use std::fmt::Debug;
use std::sync::Arc;

use futures::future::join;
use futures::{future, FutureExt, StreamExt};
use temporal_client::ClientOptionsBuilder;

use temporal_sdk::ActivityFunction;
use temporal_sdk::{WfContext, Worker, WorkflowResult};
use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::{CoreRuntime, WorkerConfigBuilder};
use time::format_description::well_known::Rfc3339;
use tokio::join;
use tokio_util::sync::CancellationToken;

async fn fuzzy_wf_def(ctx: WfContext) -> WorkflowResult<()> {
    let sigchan = ctx
        .make_signal_channel("hello")
        // .map(|x|x.input);
    ;

    let done = CancellationToken::new();
    let done_setter = done.clone();
    println!("fuzzy_wf_def");
    let chan = sigchan
        .take_until(done.cancelled())
        .for_each_concurrent(None, |action| {
            println!("time : {}", time::OffsetDateTime::now_utc().format(&Rfc3339).unwrap());
            println!("Received signal: {:?}", action);
            done_setter.cancel();
            future::ready(()).boxed()
        })
        .boxed();
    let cancel = ctx.cancelled().boxed();
    future::select(chan, cancel).await;
    // cancel.await;
    // chan.await;
    println!("fuzzy_wf_def done");
    Ok(().into())
}

#[tokio::main]
async fn main() {
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

    let worker_config = WorkerConfigBuilder::default().namespace("default").worker_build_id("event-otype-0".to_string()).task_queue("event-otype").build().unwrap();
    let core_worker = Arc::new(temporal_sdk_core::init_worker(&runtime, worker_config, client.clone()).unwrap());

    let mut worker = Worker::new_from_core(core_worker, "test?".to_string());

    worker.register_wf("test", fuzzy_wf_def);
    worker.run().await.unwrap();
}

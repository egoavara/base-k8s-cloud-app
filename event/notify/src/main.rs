use crate::agent::{create_agent, save_agent};
use crate::utils::{generate_uuid_v4, TracingActivity};
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler};
use opentelemetry_sdk::{trace, Resource};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::sync::Arc;
use std::time::Duration;
use temporal_client::ClientOptionsBuilder;
use temporal_sdk::Worker;
use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::{CoreRuntime, WorkerConfigBuilder};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, Layer};

mod agent;
mod agent_types;
mod notify;
mod notify_types;
mod utils;

#[tokio::main]
async fn main() {
    let stdout_log = tracing_subscriber::fmt::layer().pretty().with_filter(filter::LevelFilter::INFO);
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://jaeger-collector.telemetry-system.svc:4317")
                .with_timeout(Duration::from_secs(3))
                .with_protocol(opentelemetry_otlp::Protocol::Grpc),
        )
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                // .with_max_events_per_span(64)
                // .with_max_attributes_per_span(16)
                // .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new("service.name", "event/notify")])),
        )
        .install_batch(Tokio)
        .unwrap();

    let telemetry = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        // .with_filter(EnvFilter::new("graph-hmb[internal{}]=warn"))
        .with_filter(filter::LevelFilter::INFO)
        // .with_filter(filter::LevelFilter::INFO)
        ;

    let subscriber = tracing_subscriber::registry().with(telemetry).with(stdout_log);
    // tracing::subscriber::set_global_default(subscriber).unwrap();
    global::set_text_map_propagator(TraceContextPropagator::new());
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .max_lifetime(Some(Duration::from_secs(300)))
        .connect_with(
            PgConnectOptions::new()
                .host("postgresql-hl.persistence-system.svc")
                .port(5432)
                .username("postgres-user")
                .password("oOzTsQzjhP0acblx52hVauwskH6YnZKWxMOhKnvDe09quROIwGP1z4FReDDzNfNT")
                .database("public"),
        )
        .await
        .unwrap();

    let temporal_url = temporal_sdk_core::Url::parse("http://temporal-frontend-headless.event-system.svc:7233").unwrap();

    let runtime_option = TelemetryOptionsBuilder::default().attach_service_name(false).metric_prefix("event/notify".to_string()).build().unwrap();
    let runtime = CoreRuntime::new_assume_tokio(runtime_option).unwrap();
    let client_option = ClientOptionsBuilder::default()
        .identity("event/notify".to_string())
        .target_url(temporal_url)
        .client_name("event/notify-client".to_string())
        .client_version("0.1.0".to_string())
        .build()
        .unwrap();
    let client = Arc::new(client_option.connect("default", None).await.unwrap());

    let worker_config = WorkerConfigBuilder::default().namespace("default").worker_build_id("v0.0.1".to_string()).task_queue("default-queue").build().unwrap();
    let core_worker = Arc::new(temporal_sdk_core::init_worker(&runtime, worker_config, client.clone()).unwrap());

    let mut worker = Worker::new_from_core(core_worker, "test?".to_string());
    worker.insert_app_data(pool);
    worker.register_wf("create_agent", create_agent);
    worker.register_activity("generate_uuid_v4", TracingActivity::new(generate_uuid_v4));
    worker.register_activity("save_agent", TracingActivity::new(save_agent));
    worker.run().await.unwrap();
}
// temporal workflow start --type create_agent --task-queue default-queue --input '{"name": "httppost", "runtime": {"type": "Elem", "elem": {"type": "Http", "url": "http://localhost:8080", "method": "POST", "body": "{\"payload\": \"${message:literal}\"}", "headers": [{"key": "Authorization", "value": "Bearer ${secret.jwt:literal}"}]}}}'

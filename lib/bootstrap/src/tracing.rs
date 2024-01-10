use std::time::Duration;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, trace};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler};
use tracing_subscriber::{filter, Layer};
use tracing_subscriber::layer::SubscriberExt;

pub async fn init() {
    // Then pass it into pipeline builder
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint("http://jaeger-collector.telemetry.svc:4317")
            .with_timeout(Duration::from_secs(3))
            .with_protocol(opentelemetry_otlp::Protocol::Grpc)
        )
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                // .with_max_events_per_span(64)
                // .with_max_attributes_per_span(16)
                // .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", "graph-hmb")
                ]))
        )
        .install_batch(Tokio)
        .unwrap();

    let telemetry = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        // .with_filter(EnvFilter::new("graph-hmb[internal{}]=warn"))
        .with_filter(filter::LevelFilter::INFO)
        // .with_filter(filter::LevelFilter::INFO)
        ;

    let subscriber = tracing_subscriber::registry()
        .with(telemetry)
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_filter(filter::LevelFilter::INFO)
        );
    // tracing::subscriber::set_global_default(subscriber).unwrap();
    global::set_text_map_propagator(TraceContextPropagator::new());
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
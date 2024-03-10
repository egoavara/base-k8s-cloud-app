// use log::{info, warn};
//
// use rdkafka::client::ClientContext;
// use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
// use rdkafka::consumer::stream_consumer::StreamConsumer;
// use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
// use rdkafka::error::KafkaResult;
// use rdkafka::message::{Headers, Message};
// use rdkafka::topic_partition_list::TopicPartitionList;
// use rdkafka::util::get_rdkafka_version;
//
// struct CustomContext;
//
// impl ClientContext for CustomContext {}
//
// impl ConsumerContext for CustomContext {
//     fn pre_rebalance(&self, rebalance: &Rebalance) {
//         println!("Pre rebalance {:?}", rebalance);
//     }
//
//     fn post_rebalance(&self, rebalance: &Rebalance) {
//         println!("Post rebalance {:?}", rebalance);
//     }
//
//     fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
//         println!("Committing offsets: {:?}", result);
//     }
// }
//
// type LoggingConsumer = StreamConsumer<CustomContext>;
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let (_, version) = get_rdkafka_version();
//     println!("rdkafka version: {}", version);
//
//     let context = CustomContext;
//
//     let consumer: LoggingConsumer = ClientConfig::new()
//         .set("group.id", "app-server-test")
//         .set("bootstrap.servers", "kafka.persistence.svc:9092")
//         // .set("enable.partition.eof", "false")
//         .set("session.timeout.ms", "6000")
//         .set("enable.auto.commit", "true")
//         //.set("statistics.interval.ms", "30000")
//         //.set("auto.offset.reset", "smallest")
//         .set_log_level(RDKafkaLogLevel::Debug)
//         .create_with_context(context)
//         .expect("Consumer creation failed");
//
//     println!("Created consumer");
//
//     let topics = vec!["debezium-connector-postgresql.public.asset"];
//
//     consumer
//         .subscribe(topics.as_slice())
//         .expect("Can't subscribe to specified topics");
//
//     println!("Starting consumer loop");
//     loop {
//         let a = consumer.recv().await;
//         println!("Received message");
//         match a {
//             Err(e) => println!("Kafka error: {}", e),
//             Ok(m) => {
//                 let payload = match m.payload_view::<str>() {
//                     None => "",
//                     Some(Ok(s)) => s,
//                     Some(Err(e)) => {
//                         println!("Error while deserializing message payload: {:?}", e);
//                         ""
//                     }
//                 };
//                 println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
//                       m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
//                 if let Some(headers) = m.headers() {
//                     for header in headers.iter() {
//                         println!("  Header {:#?}: {:?}", header.key, header.value);
//                     }
//                 }
//                 consumer.commit_message(&m, CommitMode::Async).unwrap();
//             }
//         };
//     }
// }

use axum::extract::{MatchedPath, Request};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{response, Router};
use opentelemetry::{global, KeyValue};
use opentelemetry_http::{HeaderExtractor, HeaderInjector};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler};
use opentelemetry_sdk::{trace, Resource};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, instrument};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{filter, Layer};

#[instrument(skip_all)]
async fn test() -> impl IntoResponse {
    response::Json("Hello, World!".to_string())
}

#[tokio::main]
async fn main() {
    // Then pass it into pipeline builder
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://jaeger-collector.telemetry.svc:4317")
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
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "event-hmb",
                )])),
        )
        .install_batch(Tokio)
        .unwrap();

    let telemetry = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(filter::LevelFilter::INFO);

    let subscriber = tracing_subscriber::registry().with(telemetry).with(
        tracing_subscriber::fmt::layer()
            .pretty()
            .with_filter(filter::LevelFilter::INFO),
    );
    // tracing::subscriber::set_global_default(subscriber).unwrap();
    global::set_text_map_propagator(TraceContextPropagator::new());
    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::info!("Starting up");
    let app =
        Router::new()
            .route("/test", get(test))
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                    // HeaderMap to Hashmap string string
                    let b = request
                        .headers()
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
                        .collect::<HashMap<String, String>>();

                    let ctx = global::get_text_map_propagator(|propagator| propagator.extract(&b));

                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    let span = info_span!(
                        "http_request",
                        method = ?request.method(),
                        path = matched_path,
                        // some_other_field = tracing::field::Empty,
                    );
                    span.set_parent(ctx);

                    span
                }),
            );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql::futures_util::SinkExt;
use async_graphql::http::{ALL_WEBSOCKET_PROTOCOLS, GraphiQLSource};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{response, Router};
use axum::extract::{MatchedPath, Request, State, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum_client_ip::{InsecureClientIp, SecureClientIpSource};
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Basic;
use axum_extra::TypedHeader;
use opentelemetry::{global, KeyValue};
use opentelemetry::global::ObjectSafeLoggerProvider;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::trace::{FutureExt, TracerProvider};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, trace};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler};
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, instrument};
use tracing::log::LevelFilter;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::{EnvFilter, filter, Layer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use graph_guard::openfga::OpenFGA;
use graph_guard::{FieldGuard, FieldGuardContext, User};

use crate::entity::Story;
use crate::schema::{Query, Schema};
use crate::traits::GeneralTable;

pub mod object;
pub mod entity;
mod schema;
mod traits;

#[instrument(skip_all)]
fn get_token_from_headers(ip: InsecureClientIp, basic_auth: Option<TypedHeader<Authorization<Basic>>>, headers: &HeaderMap) -> User {
    let id = basic_auth.map(|auth| {
        auth.username().to_string()
    });
    User {
        id,
        ip: Some(ip.0),
        hint: None,
        agent: None,
    }
}
#[instrument(skip_all)]
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[instrument(skip_all)]
async fn graphql_handler(
    State(schema): State<Schema>,
    basic_auth: Option<TypedHeader<Authorization<Basic>>>,
    ip: InsecureClientIp,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let req = req.into_inner()
        .data(get_token_from_headers(ip, basic_auth, &headers))
        .data(FieldGuardContext::default());
    schema.execute(req).await.into()
}

async fn graphql_ws_handler(
    State(schema): State<Schema>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade,
) -> Response {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema.clone(), protocol)
                // .on_connection_init(on_connection_init)
                .serve()
        })
}

#[tokio::main]
async fn main() {

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

    info!("Starting up");
    let pgoption = PgConnectOptions::new()
        .host("postgres-svc.persistence.svc")
        .port(5432)
        .username("pg-svc")
        .password("a091c7e4-6c37-47ec-b279-a690da7a24d9")
        .database("svc")
        .log_statements(LevelFilter::Info)
        .log_slow_statements(LevelFilter::Warn, Duration::from_secs(1))
        ;
    let pool = PgPoolOptions::new()
        .connect_with(pgoption)
        .await
        .unwrap();

    let openfga = OpenFGA::new("http://openfga.auth.svc:8080", "01HKFPBB8QM0WA62EKD01D2MRA".to_string(), "01HKH2WQR7CKSCX0FKW9JGBN7B".to_string());
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(openfga)
        .data(Story::loader(&pool))
        .data(pool)
        // .extension(async_graphql::extensions::Tracing)
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route("/ws", get(graphql_ws_handler))
        .with_state(schema)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // HeaderMap to Hashmap string string
                    let b = request.headers().iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string())).collect::<HashMap<String, String>>();

                    let ctx = global::get_text_map_propagator(|propagator| {
                        propagator.extract(&b)
                    });

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
                })
        )
        .layer(SecureClientIpSource::ConnectInfo.into_extension());

    println!("GraphiQL IDE: http://localhost:8000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
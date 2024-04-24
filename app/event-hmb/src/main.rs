use std::collections::HashMap;
use std::net::SocketAddr;

use async_graphql::dataloader::DataLoader;
use async_graphql::http::{GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS};
use async_graphql::Schema;
use async_graphql::{EmptyMutation, EmptySubscription};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::extract::{MatchedPath, Request, State, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{response, Router};
use axum_client_ip::{InsecureClientIp, SecureClientIpSource};
use axum_extra::headers::authorization::Basic;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use opentelemetry::global;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, instrument, span, Instrument};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::schema::{EntrySchema, Query};

mod schema;

#[instrument(skip_all)]
async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[instrument(skip_all)]
async fn graphql_handler(State(schema): State<EntrySchema>, req: GraphQLRequest) -> GraphQLResponse {
    let req = req.into_inner();
    schema.execute(req).await.into()
}

async fn graphql_ws_handler(State(schema): State<EntrySchema>, protocol: GraphQLProtocol, websocket: WebSocketUpgrade) -> Response {
    websocket.protocols(ALL_WEBSOCKET_PROTOCOLS).on_upgrade(move |stream| {
        GraphQLWebSocket::new(stream, schema.clone(), protocol)
            // .on_connection_init(on_connection_init)
            .serve()
    })
}

#[tokio::main]
async fn main() {
    bootstrap::tracing::init().await;
    let pgpool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect("postgresql://pg-svc:a091c7e4-6c37-47ec-b279-a690da7a24d9@postgres-svc.persistence.svc:5432/svc")
        .await
        .unwrap();

    // let pg = bootstrap::postgres::init_svc().await;
    info!("Starting up");

    let schema = EntrySchema::build(Query, EmptyMutation, EmptySubscription).data(pgpool).finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route("/ws", get(graphql_ws_handler))
        .with_state(schema)
        .layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            // HeaderMap to Hashmap string
            let b = request.headers().iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string())).collect::<HashMap<String, String>>();

            let ctx = global::get_text_map_propagator(|propagator| propagator.extract(&b));

            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);

            let span = info_span!(
                "http_request",
                method = ?request.method(),
                path = matched_path,
                // some_other_field = tracing::field::Empty,
            );
            span.set_parent(ctx);
            span
        }))
        .layer(SecureClientIpSource::ConnectInfo.into_extension());

    println!("GraphiQL IDE: http://localhost:8000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

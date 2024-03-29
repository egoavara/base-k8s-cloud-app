use std::collections::HashMap;
use std::net::SocketAddr;

use async_graphql::dataloader::DataLoader;

use async_graphql::http::{GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS};
use async_graphql::EmptySubscription;
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

use tower_http::trace::TraceLayer;
use tracing::{info, info_span, instrument, span, Instrument, Level};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use general_table::config::{CursorConfig, GeneralTableConfig};

use graph_guard::User;

use crate::schema::{InnerLoader, Mutation, Query, Schema};

pub mod entity;
pub mod schema;

#[instrument(skip_all)]
fn get_token_from_headers(
    ip: InsecureClientIp,
    basic_auth: Option<TypedHeader<Authorization<Basic>>>,
    _headers: &HeaderMap,
) -> User {
    let id = basic_auth.map(|auth| auth.username().to_string());
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
    let req = req
        .into_inner()
        .data(get_token_from_headers(ip, basic_auth, &headers));
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
    bootstrap::tracing::init().await;
    let pg = bootstrap::postgres::init_svc().await;
    let openfga = openfga_client::init().await;
    let config = GeneralTableConfig {
        cursor_config: CursorConfig {
            default_limit: Some(100),
        },
    };
    info!("Starting up");

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        // .data(DataLoader::new(
        //     entity::public::Otype::new_loader(&pg),
        //     |x| tokio::spawn(x.instrument(span!(Level::INFO, "dataloader"))),
        // ))
        // .data(DataLoader::new(
        //     entity::public::Project::new_loader(&pg),
        //     |x| tokio::spawn(x.instrument(span!(Level::INFO, "dataloader"))),
        // ))
        .data(DataLoader::new(InnerLoader, |x| {
            tokio::spawn(x.instrument(span!(Level::INFO, "dataloader")))
        }))
        .data(config)
        .data(pg)
        // .extension(async_graphql::extensions::Tracing)
        .extension(graph_guard::GraphGuard::new(openfga))
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route("/ws", get(graphql_ws_handler))
        .with_state(schema)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // HeaderMap to Hashmap string
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
        )
        .layer(SecureClientIpSource::ConnectInfo.into_extension());

    println!("GraphiQL IDE: http://localhost:8000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

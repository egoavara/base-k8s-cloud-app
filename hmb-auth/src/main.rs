use std::net::SocketAddr;
use axum::{Router, Server};
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod api;
mod config;
mod core;
mod flow;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let serve_dir = ServeDir::new("public")
        .not_found_service(ServeFile::new("public/index.html"));

    let router = Router::new()
        .nest("/api", api::router())
        .nest("/flow", flow::router())
        .fallback_service(serve_dir)
        // .layer(Extension(config))
        ;

    let socket:SocketAddr = "0.0.0.0:8080".parse().unwrap();
    Server::bind(&socket)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
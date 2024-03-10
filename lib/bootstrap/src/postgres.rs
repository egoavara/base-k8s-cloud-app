use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::time::Duration;
use tracing::log::LevelFilter;

pub async fn init_svc() -> PgPool {
    let pgoption = PgConnectOptions::new()
        .host("postgres-svc.persistence.svc")
        .port(5432)
        .username("pg-svc")
        .password("a091c7e4-6c37-47ec-b279-a690da7a24d9")
        .database("svc")
        .log_statements(LevelFilter::Info)
        .log_slow_statements(LevelFilter::Warn, Duration::from_secs(1));
    let pool = PgPoolOptions::new().connect_with(pgoption).await.unwrap();
    pool
}

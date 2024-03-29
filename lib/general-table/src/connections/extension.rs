use async_graphql::async_trait::async_trait;
use sqlx::PgPool;

pub struct ConnectionExtension {
    pub pool: PgPool,
}

#[async_trait]
impl async_graphql::extensions::Extension for ConnectionExtension {
    as
}

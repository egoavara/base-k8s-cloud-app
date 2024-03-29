use sqlx::PgPool;

pub struct ConnectionFactory {
    pub pool: PgPool,
}

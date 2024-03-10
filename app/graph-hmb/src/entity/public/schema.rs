use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, SimpleObject)]
pub struct Schema {
    pub schema_id: Uuid,
    pub name: String,
    pub description: String,
}

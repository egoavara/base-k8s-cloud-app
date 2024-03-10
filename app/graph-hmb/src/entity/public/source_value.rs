use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, SimpleObject)]
pub struct SourceValue {
    pub value_id: Uuid,
    pub source_id: Uuid,
    pub user_name: String,
    pub relation: String,
    pub object_name: String,
}

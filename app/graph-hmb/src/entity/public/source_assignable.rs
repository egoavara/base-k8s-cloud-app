use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, SimpleObject)]
pub struct SourceAssignable {
    pub source_id: Uuid,
    pub object_id: Uuid,
}

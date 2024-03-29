use sea_query::enum_def;
use sqlx::types::Uuid;

#[enum_def(prefix = "", suffix = "Refs")]
pub struct ProjectOtype {
    pub project_id: Uuid,
    pub otype_id: Uuid,
}

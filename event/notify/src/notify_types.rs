use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{Duration, UtcOffset};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Notify {
    pub notify_id: Uuid,
    pub name: String,
    pub agent: IdOrName,
    pub message: Message,
    pub context: Option<Value>,
    pub send_at: Option<UtcOffset>,
    pub retry: Option<Retry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum Message {
    Template {
        #[serde(flatten)]
        id_or_name: IdOrName,
    },
    Literal {
        template: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Retry {
    pub count: Option<u32>,
    pub max_count: Option<u32>,
    pub interval: Option<Duration>,
    pub max_interval: Option<Duration>,
    pub backoff: Option<u32>,
    pub jitter: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdOrName {
    pub id: Option<Uuid>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNotify {
    pub name: String,
    pub agent: IdOrName,
    pub message: Message,
    pub context: Option<Value>,
    pub send_at: Option<UtcOffset>,
}

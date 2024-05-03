use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub agent_id: Uuid,
    pub name: String,
    pub runtime: AgentRuntime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentRuntime {
    Elem { elem: AgentRuntimeElem },
    Flow { elems: Vec<AgentRuntimeElem> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AgentRuntimeElem {
    Http(HttpAgent),
    Reference { agent_id: Uuid },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpAgent {
    pub url: String,
    pub method: String,
    pub query_params: Option<Vec<Pair>>,
    pub body: Option<String>,
    pub headers: Vec<Pair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pair {
    pub key: String,
    pub value: Option<String>,
    pub value_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgent {
    pub name: String,
    pub runtime: AgentRuntime,
}

use std::net::IpAddr;
use std::ops::Deref;

use async_graphql::{ErrorExtensions, Guard};

pub mod openfga;
mod field_gaurd;
mod role_gaurd;

pub use field_gaurd::*;
pub use role_gaurd::*;

const PLACEHOLDER_DEFAULT_USER: User = User {
    hint: None,
    id: None,
    ip: None,
    agent: None,
};

#[derive(Debug, Default)]
pub struct User {
    pub hint: Option<String>,
    pub id: Option<String>,
    pub ip: Option<IpAddr>,
    pub agent: Option<String>,
}

impl User {
    fn fga_user(&self) -> String {
        if let Some(id) = &self.id {
            return format!("user:id.{}", id);
        }

        "user:anonymous".to_string()
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(id) = &self.id {
            return f.write_str(id);
        }
        let mut anonymous = f.debug_struct("anonymous");
        if let Some(hint) = &self.hint {
            anonymous.field("hint", hint);
        }
        if let Some(ip) = &self.ip {
            anonymous.field("ip", ip);
        }
        if let Some(agent) = &self.agent {
            anonymous.field("agent", agent);
        }
        anonymous.finish()
    }
}




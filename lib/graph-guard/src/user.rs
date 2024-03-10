use std::net::IpAddr;

#[derive(Debug, Default, Clone)]
pub struct User {
    pub hint: Option<String>,
    pub id: Option<String>,
    pub ip: Option<IpAddr>,
    pub agent: Option<String>,
}

impl User {
    pub fn fga_notation(&self) -> String {
        if let Some(id) = &self.id {
            return format!("user:{}", id);
        }

        "user:*".to_string()
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = if let Some(id) = &self.id {
            f.debug_struct(id)
        } else {
            f.debug_struct("*")
        };
        if let Some(hint) = &self.hint {
            debug_struct.field("hint", hint);
        }
        if let Some(ip) = &self.ip {
            debug_struct.field("ip", ip);
        }
        if let Some(agent) = &self.agent {
            debug_struct.field("agent", agent);
        }
        debug_struct.finish()
    }
}

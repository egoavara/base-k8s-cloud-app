use std::collections::HashMap;

use async_graphql::{Context, Guard};
use tokio::sync::RwLock;

use crate::{PLACEHOLDER_DEFAULT_USER, User};
use crate::openfga::{CheckResponse, OpenFGA, Tuple};

#[derive(Default)]
pub struct FieldGuardContext {
    cached: RwLock<HashMap<(String, String), bool>>,
}

pub struct FieldGuard {
    pub object: String,
    pub field: String,
}

impl FieldGuard {
    pub fn new<O: Into<String>, F: Into<String>>(object: O, field: F) -> Self {
        Self {
            object: object.into(),
            field: field.into(),
        }
    }
}

#[async_trait::async_trait]
impl Guard for FieldGuard {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let fgctx = ctx.data::<FieldGuardContext>().unwrap();
        if let Some(&is_passed) = fgctx.cached.read().await.get(&(self.object.clone(), self.field.clone())) {
            return if is_passed {
                Ok(())
            } else {
                Err(async_graphql::Error::new(
                    format!("Access denied for user {} on field {}.{}", ctx.data_opt::<User>().unwrap_or_else(|| &PLACEHOLDER_DEFAULT_USER), self.object.as_str(), self.field.as_str())
                ))
            };
        }
        let mut lock = fgctx.cached.write().await;

        let openfga = ctx.data::<OpenFGA>().unwrap();

        let requester = ctx.data_opt::<User>().unwrap_or_else(|| &PLACEHOLDER_DEFAULT_USER);

        let user = requester.fga_user();
        let object = format!("field:{}.{}", self.object.as_str(), self.field.as_str());
        let result = tokio::try_join!(
           openfga.check(Tuple::new(user.clone(), "allow".to_string(), object.clone()), None),
            openfga.check(Tuple::new(user, "deny".to_string(), object), None),
        ).map_err(|e| {
            async_graphql::Error::new_with_source(
                e
            )
        })?;

        let result = match (result) {
            (_, CheckResponse::Ok { allowed: true, .. }) => {
                Err(
                    async_graphql::Error::new(
                        format!("Access denied for user {} on field {}.{}", requester, self.object.as_str(), self.field.as_str())
                    )
                )
            }
            (CheckResponse::Ok { allowed: true, .. }, CheckResponse::Ok { allowed: false, .. }) => {
                Ok(())
            }
            (CheckResponse::Ok { allowed: false, .. }, CheckResponse::Ok { allowed: false, .. }) => {
                Err(
                    async_graphql::Error::new(
                        format!("Access not allowed for user {} on field {}.{}", requester, self.object.as_str(), self.field.as_str())
                    )
                )
            }
            (allow, deny) => {
                tracing::info!("Unexpected allow: {:?}, deny: {:?}", allow, deny);
                // TODO: 텔레메트리로 왜 실패했는지 정보를 남길 것.
                Err(
                    async_graphql::Error::new(
                        format!("Access denied for user {} on field {}.{}", requester, self.object.as_str(), self.field.as_str())
                    )
                )
            }
        };
        lock.insert((self.object.clone(), self.field.clone()), result.is_ok());
        result
    }
}



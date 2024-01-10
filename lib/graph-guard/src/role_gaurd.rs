use async_graphql::{Context, Guard};
use openfga_client::{CheckResponse, OpenFGA, Tuple};
use crate::{PLACEHOLDER_DEFAULT_USER, User};

pub struct RoleGuard {
    pub role: String,
}

impl RoleGuard {
    pub fn new<R: Into<String>, F: Into<String>>(role: R) -> Self {
        Self {
            role: role.into(),
        }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let openfga = ctx.data::<OpenFGA>().unwrap();

        let requester = ctx.data_opt::<User>().unwrap_or_else(|| &PLACEHOLDER_DEFAULT_USER);

        let user = requester.fga_user();
        let object = format!("role:{}", self.role.as_str());

        let result = tokio::try_join!(
           openfga.check(Tuple::new(user.clone(), "allow".to_string(), object.clone()), None),
            openfga.check(Tuple::new(user, "deny".to_string(), object), None),
        ).map_err(|e| {
            async_graphql::Error::new_with_source(
                e
            )
        })?;

        match (result) {
            (_, CheckResponse::Ok { allowed: true, .. }) => {
                return Err(
                    async_graphql::Error::new(
                        format!("Access denied for user {} on role {}", requester, self.role.as_str())
                    )
                );
            }
            (CheckResponse::Ok { allowed: true, .. }, CheckResponse::Ok { allowed: false, .. }) => {
                return Ok(());
            }
            (CheckResponse::Ok { allowed: false, .. }, CheckResponse::Ok { allowed: false, .. }) => {
                return Err(
                    async_graphql::Error::new(
                        format!("Access not allowed for user {} on role {}", requester, self.role.as_str())
                    )
                );
            }
            (allow, deny) => {
                tracing::info!("Unexpected allow: {:?}, deny: {:?}", allow, deny);
                // TODO: 텔레메트리로 왜 실패했는지 정보를 남길 것.
                return Err(
                    async_graphql::Error::new(
                        format!("Access denied for user {} on role {}", requester, self.role.as_str())
                    )
                );
            }
        }
    }
}



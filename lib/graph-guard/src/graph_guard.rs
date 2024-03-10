use std::cell::RefCell;
use std::sync::Arc;

use async_graphql::extensions::{ExtensionContext, NextParseQuery, NextValidation};
use async_graphql::parser::types::ExecutableDocument;
use async_graphql::{ServerError, ServerResult, ValidationResult, Variables};
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tracing::Instrument;

use openfga_client::{CheckResponse, OpenFGA};

use crate::directive_searcher::{DirectiveSearcher, FoundRebacTypeDirective};
use crate::{RebacTypeDirective, User};

pub struct GraphGuard {
    openfga: OpenFGA,
}

impl GraphGuard {
    pub fn new<T: Into<OpenFGA>>(openfga: T) -> Self {
        Self {
            openfga: openfga.into(),
        }
    }
}

pub struct GraphGuardExtension {
    openfga: OpenFGA,
    shared: Mutex<RefCell<Vec<FoundRebacTypeDirective>>>,
}

impl async_graphql::extensions::ExtensionFactory for GraphGuard {
    fn create(&self) -> Arc<dyn async_graphql::extensions::Extension> {
        Arc::new(GraphGuardExtension {
            openfga: self.openfga.clone(),
            shared: Mutex::new(RefCell::new(Vec::new())),
        })
    }
}

#[async_trait::async_trait]
impl async_graphql::extensions::Extension for GraphGuardExtension {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let parsed = next.run(ctx, query, variables).await?;
        let directives = DirectiveSearcher::new(&ctx.schema_env.registry)
            .search(&parsed)
            .end()?;
        self.shared.lock().await.replace(directives);
        Ok(parsed)
    }
    #[tracing::instrument(skip_all)]
    async fn validation(
        &self,
        ctx: &ExtensionContext<'_>,
        next: NextValidation<'_>,
    ) -> async_graphql::Result<ValidationResult, Vec<ServerError>> {
        let span = tracing::Span::current();
        let user = ctx
            .data_opt::<User>()
            .cloned()
            .unwrap_or_else(|| Default::default());
        let mut directives = JoinSet::new();
        for x in self.shared.lock().await.take() {
            let tuple = x.type_directive.tuple(&user);
            let openfga = self.openfga.clone();
            directives.spawn(
                async move {
                    openfga
                        .check(tuple.clone(), None)
                        .await
                        .map(|r| (tuple, x, r))
                        .map_err::<crate::Error, _>(|err| err.into())
                }
                .instrument(span.clone()),
            );
        }
        let mut errors: Vec<ServerError> = Vec::new();
        while let Some(result) = directives.join_next().await {
            let result = match result {
                Ok(ok) => ok,
                Err(_) => {
                    continue;
                }
            };
            let (
                tuple,
                FoundRebacTypeDirective {
                    pos,
                    type_directive,
                },
                result,
            ) = match result {
                Ok(ok) => ok,
                Err(err) => {
                    errors.push(ServerError::new(
                        format!("Access denied for user, {}", err),
                        None,
                    ));
                    continue;
                }
            };
            match (type_directive, result) {
                (
                    RebacTypeDirective {
                        result: expected, ..
                    },
                    CheckResponse::Ok {
                        allowed: actual, ..
                    },
                ) if expected == actual => {}
                (
                    RebacTypeDirective {
                        result: expected, ..
                    },
                    CheckResponse::Ok {
                        allowed: actual, ..
                    },
                ) => {
                    errors.push(ServerError::new(
                        format!(
                            "Access denied for user, {:?} expected {:?}, actual {:?}",
                            tuple.to_string(),
                            if expected { "allow" } else { "deny" },
                            if actual { "allow" } else { "deny" },
                        ),
                        Some(pos),
                    ));
                }
                _ => {}
            }
        }
        if errors.is_empty() {
            Ok(next.run(ctx).await?)
        } else {
            Err(errors)
        }
    }
}

use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use async_graphql::{Request, Response, ServerError, ServerResult};
use async_graphql::async_trait::async_trait;
use async_graphql::extensions::{Extension, ExtensionContext, ExtensionFactory, NextExecute, NextPrepareRequest};
use sqlx::{PgPool, Postgres, Transaction};
use tokio::sync::{Mutex, MutexGuard};

pub struct PgManagerExtension(pub PgPool);

#[derive(Clone)]
pub struct PgManager {
    pool: PgPool,
    tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
}

pub struct PgTx<'t>(MutexGuard<'t, Option<Transaction<'static, Postgres>>>);

impl ExtensionFactory for PgManagerExtension {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(PgManager {
            pool: self.0.clone(),
            tx: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait]
impl Extension for PgManager {
    async fn prepare_request(&self, ctx: &ExtensionContext<'_>, request: Request, next: NextPrepareRequest<'_>) -> ServerResult<Request> {
        // The code here will be un before the prepare_request is executed, just after the request lifecycle hook.
        let result = next.run(ctx, request.data(self.clone())).await;
        // The code here will be run just after the prepare_request
        result
    }


    async fn execute(&self, ctx: &ExtensionContext<'_>, operation_name: Option<&str>, next: NextExecute<'_>) -> Response {
        // Before starting resolving the whole query
        let result = next.run(ctx, operation_name).await;
        // After resolving the whole query
        let err = {
            let otx = self.tx.lock().await.take();
            match (otx, result.is_ok()) {
                (Some(tx), true) => {
                    tx.commit()
                      .await
                      .map_err(|err| ServerError {
                          message: "Commit failed".to_string(),
                          source: None,
                          locations: vec![],
                          path: vec![],
                          extensions: None, // TODO : Ext error
                      })
                      .err()
                }
                (Some(tx), false) => {
                    tx.rollback()
                      .await
                      .map_err(|err| ServerError {
                          message: "Rollback failed".to_string(),
                          source: None,
                          locations: vec![],
                          path: vec![],
                          extensions: None, // TODO : Ext error
                      })
                      .err()
                }
                _ => None
            }
        };
        if let Some(err) = err {
            return Response::from_errors(vec![err]);
        }
        result
    }
}

impl PgManager {
    pub async fn try_use_tx<'a>(&'a self) -> Option<PgTx<'a>> {
        let mut otx = self.tx.lock().await;
        if otx.is_none() {
            return None;
        }
        Some(PgTx(otx))
    }
    pub async fn use_tx<'a>(&'a self) -> Result<PgTx<'a>, sqlx::Error> {
        Ok(PgTx({
            let mut otx = self.tx.lock().await;
            if otx.is_none() {
                let tx = self.pool.begin().await?;
                otx.replace(tx);
            }
            otx
        }))
    }
}

impl<'t> Deref for PgTx<'t> {
    type Target = Transaction<'static, Postgres>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.0.as_ref().unwrap_unchecked()
        }
    }
}

impl<'t> DerefMut for PgTx<'t> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.0.as_mut().unwrap_unchecked()
        }
    }
}
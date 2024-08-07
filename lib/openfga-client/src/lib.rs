use std::collections::HashMap;

use opentelemetry_http::HeaderInjector;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub type ConditionContext = HashMap<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextualTuples {
    tuple_keys: Vec<ContextualTuple>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tuple {
    pub user: String,
    pub relation: String,
    pub object: String,
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "is {} related to {} as {}?",
            self.user, self.object, self.relation
        )
    }
}

impl Tuple {
    pub fn new(user: String, relation: String, object: String) -> Self {
        Self {
            user,
            relation,
            object,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    name: String,
    context: ConditionContext,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextualTuple {
    user: String,
    relation: String,
    object: String,
    condition: Option<Vec<Condition>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckRequest {
    authorization_model_id: String,
    tuple_key: Tuple,
    contextual_tuples: Option<ContextualTuples>,
    context: Option<ConditionContext>,
}

#[derive(Debug)]
pub enum CheckResponse {
    Ok { allowed: bool, resolution: String },
    InvalidInput { code: String, message: String },
    IncorrectPath { code: String, message: String },
    TransactionalConflict { code: String, message: String },
    InternalServerError { code: String, message: String },
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Reqwest: {0}")]
    Reqwest(reqwest::Error),

    #[error("OpenFGA::Check unreachable status code: {0}")]
    CheckUnexpectedStatusCode(StatusCode),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CheckResponseOk {
    allowed: bool,
    resolution: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CheckResponseFail {
    code: String,
    message: String,
}

#[derive(Clone, Debug)]
pub struct OpenFGA {
    url: String,
    store_id: String,
    authorization_model_id: String,
    client: Client,
}

impl OpenFGA {
    pub fn new<URL: Into<String>>(
        url: URL,
        store_id: String,
        authorization_model_id: String,
    ) -> Self {
        Self {
            url: url.into(),
            store_id,
            authorization_model_id,
            client: Client::new(),
        }
    }
    #[tracing::instrument(skip_all, fields(tuple.user = tuple_key.user, tuple.relation = tuple_key.relation, tuple.object = tuple_key.object))]
    pub async fn check(
        &self,
        tuple_key: Tuple,
        context: Option<ConditionContext>,
    ) -> Result<CheckResponse, Error> {
        let span = tracing::Span::current();
        let mut request = self
            .client
            .post(format!("{}/stores/{}/check", &self.url, &self.store_id))
            .json(&CheckRequest {
                authorization_model_id: self.authorization_model_id.clone(),
                tuple_key: tuple_key.clone(),
                contextual_tuples: None,
                context,
            })
            .build()?;
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&span.context(), &mut HeaderInjector(request.headers_mut()));
        });
        let response = self.client.execute(request).await?;
        let result = match response.status() {
            StatusCode::OK => {
                let body: CheckResponseOk = response.json().await?;
                Ok(CheckResponse::Ok {
                    allowed: body.allowed,
                    resolution: body.resolution,
                })
            }
            StatusCode::BAD_REQUEST => {
                let body: CheckResponseFail = response.json().await?;
                Ok(CheckResponse::InvalidInput {
                    code: body.code,
                    message: body.message,
                })
            }
            StatusCode::NOT_FOUND => {
                let body: CheckResponseFail = response.json().await?;
                Ok(CheckResponse::IncorrectPath {
                    code: body.code,
                    message: body.message,
                })
            }
            StatusCode::CONFLICT => {
                let body: CheckResponseFail = response.json().await?;
                Ok(CheckResponse::TransactionalConflict {
                    code: body.code,
                    message: body.message,
                })
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                let body: CheckResponseFail = response.json().await?;
                Ok(CheckResponse::InternalServerError {
                    code: body.code,
                    message: body.message,
                })
            }
            _ => Err(Error::CheckUnexpectedStatusCode(response.status())),
        };
        info!("OpenFGA::Check {:?} is {:?}", tuple_key, result);
        result
    }
}

pub async fn init() -> OpenFGA {
    OpenFGA::new(
        "http://openfga.auth.svc:8080",
        "01HKFPBB8QM0WA62EKD01D2MRA".to_string(),
        "01HM13RRGHG855RS2QVGFECF6Y".to_string(),
    )
}

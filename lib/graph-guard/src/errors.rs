use async_graphql::ServerError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("graph_guard::rebac Duplicate argument {0}")]
    DirectiveDuplicateArgument(String),
    #[error("graph_guard::rebac Argument {0} must be a string")]
    DirectiveArgumentMustBeAString(String),
    #[error("graph_guard::rebac Argument {0} must be a boolean")]
    DirectiveArgumentMustBeABoolean(String),
    #[error("graph_guard::rebac Unknown argument {0} with value {1:?}")]
    DirectiveUnknownArgument(String, async_graphql_value::ConstValue),
    #[error("graph_guard::rebac No required field {0:?}")]
    DirectiveNoRequiredField(Vec<String>),

    #[error("graph_guard::runtime Unavailable Operation type {0}")]
    RuntimeUnavailableOperationType(String),
    #[error("graph_guard::runtime Unknown type {otype}")]
    RuntimeUnknownType { otype: String },
    #[error("graph_guard::runtime Unknown field {field} type {otype}")]
    RuntimeUnknownTypeField { otype: String, field: String },

    #[error(transparent)]
    OpenFGA(#[from] openfga_client::Error),
}

impl From<Error> for ServerError {
    fn from(err: Error) -> Self {
        ServerError::new(err.to_string(), None)
    }
}

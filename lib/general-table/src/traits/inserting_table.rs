use async_graphql::async_trait::async_trait;
use async_graphql::{InputType, OutputType};

#[async_trait]
pub trait InsertingTable: Sized + InputType {
    type Returning: OutputType;
    type Failure: OutputType;

    async fn insert<'ctx>(
        &self,
        ctx: &async_graphql::Context<'ctx>,
    ) -> Result<Self::Returning, InsertingError<Self::Failure>>;
}

pub enum InsertingError<Failure> {
    Throw(Failure),
    Unexpected(async_graphql::Error),
}

impl<Failure> From<async_graphql::Error> for InsertingError<Failure> {
    fn from(e: async_graphql::Error) -> Self {
        InsertingError::Unexpected(e)
    }
}

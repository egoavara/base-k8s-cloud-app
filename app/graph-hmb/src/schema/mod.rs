pub use mutation::*;
pub use query::*;

mod mutation;
mod query;

pub type Schema = async_graphql::Schema<Query, Mutation, async_graphql::EmptySubscription>;

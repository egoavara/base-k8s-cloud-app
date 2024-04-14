pub use query::Query;

mod query;

pub type EntrySchema =
    async_graphql::Schema<Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

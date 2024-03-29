use crate::traits::{InsertingTable, TableDefinition, UpdatingTable};
use async_graphql::async_trait;

#[async_trait::async_trait]
pub trait TableMutation {
    type Table: TableDefinition;

    type Inserting: InsertingTable;

    type Updating: UpdatingTable;

    // type Upserting: UpsertingTable;

    async fn insert<'a>(
        ctx: &async_graphql::Context<'a>,
        input: Self::Inserting,
    ) -> Result<Self::Inserting, async_graphql::Error>;
}

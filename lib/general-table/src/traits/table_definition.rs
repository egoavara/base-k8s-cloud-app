use std::hash::Hash;

use async_graphql::async_trait::async_trait;
use async_graphql::connection::{Connection, EmptyFields};
use async_graphql::dataloader::Loader;
use async_graphql::{Context, OutputType};
use sea_query::{Iden, IntoColumnRef, SimpleExpr};
use serde_json::Value;


use crate::object::Cursor;
use crate::traits::{FilterTable, SortingTable};

#[async_trait]
pub trait TableDefinition: OutputType + Sized + Send + Sync + 'static {
    type Id: Send + Sync + Hash + Eq + Clone + 'static;

    type References: IntoColumnRef + Clone + Eq + Hash + Iden;

    type Loader: Loader<Self::Id>;

    type Filter: FilterTable<Table = Self>;

    type Sorting: SortingTable<Table = Self>;

    fn table() -> Self::References;

    fn id_column() -> Self::References;

    fn encode_field(&self, key: Self::References) -> Value;

    fn decode_field(key: Self::References, value: Value) -> SimpleExpr;

    async fn find<'a>(
        ctx: &Context<'a>,
        cursor: Cursor,
        filter: Self::Filter,
        sorting: Self::Sorting,
    ) -> Result<Connection<String, Self, EmptyFields, EmptyFields>, async_graphql::Error>;
}

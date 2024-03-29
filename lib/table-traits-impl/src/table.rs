use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;

use async_graphql::Context;
use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::cursor::Cursor;

#[async_trait]
pub trait Table: Sized {
    type Id: Send + Sync + Hash + Eq + Clone + 'static;

    type Filter;

    type Sorting;

    type Cursor: DeserializeOwned;

    fn schema() -> Cow<'static, str> {
        Cow::Borrowed("public")
    }

    fn table() -> Cow<'static, str>;

    fn id_column() -> Cow<'static, str>;

    async fn find<'a>(
        ctx: &Context<'a>,
        cursor: Self::Cursor,
        filter: Self::Filter,
        sorting: Self::Sorting,
    ) -> HashMap<Self::Id, Self>;

    fn new_cursor(
        after: Option<String>,
        first: Option<u32>,
        before: Option<String>,
        last: Option<u32>,
    ) -> Result<Cursor<Self::Cursor>, serde_qs::Error> {
        Cursor::<Self::Cursor>::new(after, first, before, last)
    }
}

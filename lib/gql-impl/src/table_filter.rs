use std::hash::Hash;

use async_trait::async_trait;
use sea_query::Condition;

use crate::types::Context;
use crate::{FieldMetadata, Table};

#[async_trait]
pub trait TableFilter: Sized {
    type Table: Table;

    fn filter_fields() -> Vec<<Self::Table as FieldMetadata>::Field>;

    fn by_id(id: <Self::Table as Table>::Id) -> Self;

    fn to_condition<'a, 'b>(&self, state: &'a mut Context<'b, Self::Table>) -> Condition;
}

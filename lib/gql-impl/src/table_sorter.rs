use async_trait::async_trait;
use sea_query::{Condition, NullOrdering, Order, SimpleExpr};

use crate::types::Context;
use crate::{FieldMetadata, Table};

#[async_trait]
pub trait TableSorter: Sized {
    type Table: Table;

    fn sorter_fields() -> Vec<<Self::Table as FieldMetadata>::Field>;

    fn to_after_condition<'a, 'b>(&self, cursor: <Self::Table as Table>::Cursor, state: &'a mut Context<'b, Self::Table>) -> Condition;

    fn to_before_condition<'a, 'b>(&self, cursor: <Self::Table as Table>::Cursor, state: &'a mut Context<'b, Self::Table>) -> Condition;

    fn to_has_after_condition<'a, 'b>(&self, cursor: <Self::Table as Table>::Cursor, state: &'a mut Context<'b, Self::Table>) -> Condition;

    fn to_has_before_condition<'a, 'b>(&self, cursor: <Self::Table as Table>::Cursor, state: &'a mut Context<'b, Self::Table>) -> Condition;

    //
    fn to_order<'a, 'b>(&self, state: &'a mut Context<'b, Self::Table>) -> Vec<(SimpleExpr, Order, Option<NullOrdering>)>;
}

use std::borrow::Borrow;
use std::hash::Hash;

use async_trait::async_trait;
use sea_query::{Alias, ColumnRef, IntoIden, NullOrdering, SelectStatement, TableRef};

use crate::page::{Page, PageByCursor};
use crate::types::{Config, Parameter, State, DEFAULT_CONFIG};
use crate::{Cursor, Field, FieldGetter, FieldMetadata, TableFilter, TableSorter};

#[async_trait]
pub trait Table: Sized + FieldMetadata + FieldGetter {
    type Id: Send + Sync + Hash + Eq + Clone + 'static;

    type Filter: TableFilter<Table = Self>;

    type Sorter: TableSorter<Table = Self>;

    type Cursor: Cursor;

    fn table() -> TableRef;

    fn id_fields() -> Vec<Self::Field>;

    fn to_cursor(&self) -> Self::Cursor;
}

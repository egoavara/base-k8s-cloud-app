use crate::page::Page;
use crate::Table;
use sea_query::{Alias, ColumnRef, IntoColumnRef, IntoIden, IntoTableRef, TableRef};
use std::marker::PhantomData;

use crate::types::Config;

impl Config {
    pub fn context_as<T: Table, A: Into<Option<String>>>(&self, alias: A) -> Context<T> {
        let alias = alias.into().map(|alias| Alias::new(alias));
        Context { config: self, alias, _phantom: PhantomData }
    }
    pub fn execute<T: Table>(&self, page: Page<T::Cursor>, filter: T::Filter, sorter: T::Sorter) -> State<T> {
        State {
            context: Context {
                config: self,
                alias: None,
                _phantom: PhantomData,
            },
            parameter: Parameter { page, filter, sorter },
        }
    }
}

pub struct State<'a, T: Table> {
    pub context: Context<'a, T>,
    pub parameter: Parameter<T>,
}

pub struct Context<'a, T: Table> {
    pub config: &'a Config,
    pub alias: Option<Alias>,
    _phantom: PhantomData<fn() -> T>,
}

impl<'a, T: Table> Context<'a, T> {
    pub fn table_ref(&self, table: impl IntoTableRef) -> TableRef {
        let table = table.into_table_ref();
        match self.alias.clone() {
            Some(alias) => table.alias(alias),
            None => table,
        }
    }
    pub fn column_ref(&self, column: impl IntoColumnRef) -> ColumnRef {
        let column = column.into_column_ref();
        match self.alias.clone() {
            Some(alias) => match column {
                ColumnRef::Column(column) | ColumnRef::TableColumn(_, column) | ColumnRef::SchemaTableColumn(_, _, column) => ColumnRef::TableColumn(alias.into_iden(), column.into_iden()),
                ColumnRef::Asterisk | ColumnRef::TableAsterisk(_) => ColumnRef::TableAsterisk(alias.into_iden()),
            },
            None => column,
        }
    }
}
pub struct Parameter<T: Table> {
    pub page: Page<T::Cursor>,
    pub filter: T::Filter,
    pub sorter: T::Sorter,
}

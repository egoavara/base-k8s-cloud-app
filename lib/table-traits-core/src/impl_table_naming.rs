use crate::derive_table::{Case, TableNaming};
use inflector::Inflector;
use lazy_static::lazy_static;
use proc_macro2::Ident;
use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"(?<table>.+)(@(?<column>.+))?").unwrap();
}

impl TableNaming {
    pub(crate) fn to_schema_name<'s>(
        &self,
        schema: &'s Option<String>,
        data: &'s str,
    ) -> Cow<'s, str> {
        match schema {
            Some(schema) => Cow::Borrowed(schema.as_str()),
            None => Cow::Owned(self.to_db_case(data)),
        }
    }
    pub(crate) fn to_table_name<'s>(
        &self,
        table: &'s Option<String>,
        data: &'s str,
    ) -> Cow<'s, str> {
        match table {
            Some(table) => Cow::Borrowed(table.as_str()),
            None => Cow::Owned(self.to_db_case(data)),
        }
    }
    pub(crate) fn to_field_type_name<'s>(&self, data: &'s str) -> Cow<'s, str> {
        PATTERN.replace(data, self.field.as_str())
    }
    pub(crate) fn to_table_filter_type_name<'s>(&self, data: &'s str) -> Cow<'s, str> {
        PATTERN.replace(data, self.filter.as_str())
    }
    pub(crate) fn to_table_sorter_type_name<'s>(&self, data: &'s str) -> Cow<'s, str> {
        PATTERN.replace(data, self.sorter.as_str())
    }
    pub(crate) fn to_table_sorter_elem_type_name<'s>(&self, data: &'s str) -> Cow<'s, str> {
        PATTERN.replace(data, self.sorter_elem.as_str())
    }
    pub(crate) fn to_cursor_type_name<'s>(&self, data: &'s str) -> Cow<'s, str> {
        PATTERN.replace(data, self.cursor.as_str())
    }
    pub(crate) fn to_filter_type_ident(&self, table: &Ident, column: &Ident) -> Ident {
        Ident::new(
            &self.to_locality_filter_type_name(&table.to_string(), &column.to_string()),
            column.span(),
        )
    }
    pub(crate) fn to_locality_filter_type_name(&self, table: &str, column: &str) -> String {
        let temp = format!("{}@{}", table, column);
        PATTERN
            .replace(&temp, self.locality_filter.as_str())
            .to_string()
    }
    pub(crate) fn to_locality_sorter_type_name(&self, table: &str, column: &str) -> String {
        let temp = format!("{}@{}", table, column);
        PATTERN
            .replace(&temp, self.locality_sorter.as_str())
            .to_string()
    }
    pub(crate) fn to_field_ident<'s>(&self, ident: &Ident) -> Ident {
        let temp = self.to_field_case(&ident.to_string());
        Ident::new(&temp, ident.span())
    }
    pub(crate) fn to_field_case<'s>(&self, data: &'s str) -> String {
        match self.field_case {
            Case::Camel => data.to_camel_case(),
            Case::Pascal => data.to_pascal_case(),
            Case::Snake => data.to_snake_case(),
        }
    }
    pub(crate) fn to_enum_ident<'s>(&self, ident: &Ident) -> Ident {
        let temp = self.to_enum_case(&ident.to_string());
        Ident::new(&temp, ident.span())
    }
    pub(crate) fn to_enum_case<'s>(&self, data: &'s str) -> String {
        match self.enum_case {
            Case::Camel => data.to_camel_case(),
            Case::Pascal => data.to_pascal_case(),
            Case::Snake => data.to_snake_case(),
        }
    }
    pub(crate) fn to_db_case<'s>(&self, data: &'s str) -> String {
        match self.db_case {
            Case::Camel => data.to_camel_case(),
            Case::Pascal => data.to_pascal_case(),
            Case::Snake => data.to_snake_case(),
        }
    }
}

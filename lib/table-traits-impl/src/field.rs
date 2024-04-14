use sea_query::{ColumnRef, IntoColumnRef, IntoIden, IntoTableRef, TableRef};
use std::hash::Hash;

pub trait Field: Send + Sync + Clone + Copy + Eq + PartialEq + Hash + 'static {
    fn column_ident(&self) -> ::sea_query::Alias;
    fn column(&self, table_ref: impl IntoTableRef) -> ColumnRef {
        match table_ref.into_table_ref() {
            TableRef::Table(table) => {
                ColumnRef::TableColumn(table, self.column_ident().into_iden())
            }
            TableRef::SchemaTable(schema, table) => {
                ColumnRef::SchemaTableColumn(schema, table, self.column_ident().into_iden())
            }
            TableRef::DatabaseSchemaTable(database, schema, table) => {
                unimplemented!("DatabaseSchemaTableColumn not support, normally it can't be reached do not call this function manually")
            }
            TableRef::TableAlias(_, alias)
            | TableRef::SchemaTableAlias(_, _, alias)
            | TableRef::DatabaseSchemaTableAlias(_, _, _, alias)
            | TableRef::SubQuery(_, alias)
            | TableRef::ValuesList(_, alias)
            | TableRef::FunctionCall(_, alias) => {
                ColumnRef::TableColumn(alias, self.column_ident().into_iden())
            }
        }
    }
}

pub trait FieldMetadata {
    type Field: Field;
    fn fields() -> Vec<Self::Field>;
}

pub trait FieldGetter: FieldMetadata {
    fn get_field_value_graphql(&self, field: Self::Field) -> async_graphql_value::ConstValue {
        async_graphql_value::ConstValue::from_json(self.get_field_value_json(field)).unwrap()
    }
    fn get_field_value_json(&self, field: Self::Field) -> serde_json::Value;
}

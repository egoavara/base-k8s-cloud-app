use crate::table_attr::TableAttr;
use proc_macro2::Ident;
use syn::{Data, DeriveInput, Field};

#[derive(Debug)]
pub struct TableFields<'a> {
    pub inner: Vec<TableField<'a>>,
    id_field_idx: usize,
}

#[derive(Debug)]
pub struct TableField<'a> {
    pub field: &'a Field,
    pub attr: TableAttr,
}

impl<'a> TableFields<'a> {
    pub fn new<'b: 'a>(_crate_ident: &Ident, ast: &'b DeriveInput) -> Result<Self, String> {
        let inner = Self::to_table_fields(ast)?;
        let id_field_idx = inner
            .iter()
            .enumerate()
            .find(|(_, field)| field.attr.id)
            .map(|(idx, _)| idx)
            .ok_or_else(|| "id field not found".to_string())?;

        Ok(Self {
            inner,
            id_field_idx,
        })
    }
    pub fn id_field(&self) -> &TableField<'a> {
        &self.inner[self.id_field_idx]
    }

    fn to_table_fields(ast: &DeriveInput) -> Result<Vec<TableField>, String> {
        match &ast.data {
            Data::Struct(data) => data
                .fields
                .iter()
                .map(|field| {
                    field
                        .attrs
                        .iter()
                        .find(|attr| attr.path().is_ident("table"))
                        .map_or(Ok(None), |attr| attr.parse_args::<TableAttr>().map(Some))
                        .map_err(|err| err.to_string())
                        .map(|attr| attr.unwrap_or_default())
                        .map(|attr| TableField { field, attr })
                })
                .collect::<Result<Vec<_>, _>>(),
            Data::Enum(_data) => {
                panic!("")
            }
            Data::Union(_data) => {
                panic!("")
            }
        }
    }
}

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SorterKind {
    Asc,
    Desc,
    Values,
    // NullFirstAsc,
    // NullFirstDesc,
    // NullLastAsc,
    // NullLastDesc,
    // Expr(),
    // Values(Vec<serde_json::Value>),
}

impl SorterKind {
    pub fn all() -> Vec<SorterKind> {
        vec![SorterKind::Asc, SorterKind::Desc]
    }
    pub fn field_ident(&self, span: Span) -> Ident {
        match self {
            SorterKind::Asc => Ident::new("asc", span),
            SorterKind::Desc => Ident::new("desc", span),
            SorterKind::Values => Ident::new("values", span),
        }
    }
    pub fn enum_value(&self, span: Span) -> Ident {
        match self {
            SorterKind::Asc => Ident::new("Asc", span),
            SorterKind::Desc => Ident::new("Desc", span),
            SorterKind::Values => Ident::new("Desc", span),
        }
    }
    pub fn sea_query_order_enum(&self, span: Span) -> Ident {
        match self {
            SorterKind::Asc => Ident::new("Asc", span),
            SorterKind::Desc => Ident::new("Desc", span),
            SorterKind::Values => Ident::new("Values", span),
        }
    }
}

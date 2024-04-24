use proc_macro2::{Ident, TokenStream};
use syn::punctuated::Punctuated;
use syn::{parse_quote, DataStruct, Fields, FieldsNamed, Visibility};

use crate::derive_table::{Column, ColumnFilterWith, ColumnSorterWith, Table};
use crate::syntax::{SyntaxFilter, SyntaxSorter};
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

pub struct TokenSorter<'a> {
    pub(crate) table: &'a Table,
    pub(crate) column: &'a Column,
    pub(crate) with: &'a ColumnSorterWith,
}

impl<'a> TokenSorter<'a> {
    pub(crate) fn ident(&self) -> Ident {
        self.table
            .naming
            .to_filter_type_ident(&self.table.ident, &self.column.ident.as_ref().unwrap())
    }
    fn mimic_syntax(&self) -> SyntaxSorter {
        let mut attrs = Vec::new();
        attrs.push(parse_quote! {#[derive(Default, Debug, Default)]});
        SyntaxSorter::new_simple(
            attrs,
            Visibility::Public(Default::default()),
            self.ident(),
            self.column.ty.clone(),
            self.with.clone(),
        )
    }
}

impl<'a> ToTokenWrapperSupport for TokenSorter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        self.mimic_syntax().to_tokens(tokens, crate_location);
    }
}

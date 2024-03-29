use proc_macro2::{Ident, TokenStream};
use syn::parse::ParseStream;
use syn::{Token, Type};

use crate::sorter_kind::SorterKind;

#[derive(Debug)]
pub enum SorterVariants {
    Implemented(Token![impl], SorterKind),
}

impl SorterVariants {
    pub fn parse(parser: ParseStream) -> syn::Result<SorterVariants> {
        let impl_token = parser.parse::<Token![impl]>()?;
        let fields = parser.parse::<SorterKind>()?;
        Ok(SorterVariants::Implemented(impl_token, fields))
    }
    pub fn implemented_sorter_expr(&self, crate_ident: &Ident) -> TokenStream {
        match self {
            SorterVariants::Implemented(_, kind) => kind.implemented_filters_expr(crate_ident),
        }
    }
    pub fn activated_sorter_match_arm(&self, crate_ident: &Ident) -> TokenStream {
        match self {
            SorterVariants::Implemented(_, kind) => kind.activated_sorter_match_arm(crate_ident),
        }
    }
    pub fn sorter_value_match_arm(&self, crate_ident: &Ident) -> TokenStream {
        match self {
            SorterVariants::Implemented(_, kind) => kind.sorter_value_match_arm(crate_ident),
        }
    }
    pub fn to_variant(&self, _crate_ident: &Ident, _target_type: &Type) -> TokenStream {
        match self {
            SorterVariants::Implemented(_, kind) => kind.to_variant(),
        }
    }
}

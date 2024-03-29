use proc_macro2::{Ident, TokenStream};
use quote::{quote};
use syn::punctuated::Punctuated;
use syn::{FieldsNamed, Token, Type};

use crate::filter_kind::FilterKind;

#[derive(Debug)]
pub enum FilterFields {
    Named(FieldsNamed),
    Implemented(Token![impl], Punctuated<FilterKind, Token![+]>),
}

impl FilterFields {
    pub fn to_implemented_filters_exprs(
        &self,
        crate_ident: &Ident,
    ) -> syn::Result<Vec<TokenStream>> {
        Ok(match self {
            FilterFields::Implemented(_, fields) => fields
                .iter()
                .map(|x| x.implemented_filters_expr(crate_ident))
                .collect::<Vec<_>>(),
            FilterFields::Named(_fields) => unimplemented!("Named fields not implemented yet"),
        })
    }
    pub fn to_activated_filters_exprs(&self, crate_ident: &Ident) -> syn::Result<Vec<TokenStream>> {
        Ok(match self {
            FilterFields::Implemented(_, fields) => fields
                .iter()
                .map(|x| x.activated_filters_expr(crate_ident))
                .collect::<Vec<_>>(),
            FilterFields::Named(_fields) => unimplemented!("Named fields not implemented yet"),
        })
    }
    pub fn to_filter_value_match_arms(&self, crate_ident: &Ident) -> syn::Result<Vec<TokenStream>> {
        Ok(match self {
            FilterFields::Implemented(_, fields) => fields
                .iter()
                .map(|x| x.filter_value_match_arm(crate_ident))
                .collect::<Vec<_>>(),
            FilterFields::Named(_fields) => unimplemented!("Named fields not implemented yet"),
        })
    }
    pub fn to_struct_fields(
        &self,
        crate_ident: &Ident,
        target_ty: &Type,
    ) -> syn::Result<Vec<TokenStream>> {
        Ok(match self {
            FilterFields::Named(_fields) => {
                unimplemented!("Named fields not implemented yet")
            }
            FilterFields::Implemented(_, fields) => fields
                .iter()
                .map(|x| {
                    let field_ident = x.field_ident();
                    let field_ty = x.field_ty(crate_ident, target_ty);
                    quote!(#field_ident: #field_ty)
                })
                .collect::<Vec<_>>(),
        })
    }
}

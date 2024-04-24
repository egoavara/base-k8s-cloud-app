use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Token, Type};

use crate::derive_table::ColumnSorterWith;
use crate::enum_sorter_kind::SorterKind;
use crate::syntax::sorter_kind::SyntaxSorterKind;
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

pub struct SyntaxSorterOption {
    pub for_token: Token![for],
    pub target_ty: Type,
    pub impl_token: Token![impl],
    pub inner: ColumnSorterWith,
}
impl SyntaxSorterOption {
    pub fn complex_order_rs(&self, crate_location: CrateLocation) -> Option<TokenStream> {
        if self.inner.is_simple_order() {
            return None;
        }

        Some(match (self.inner.asc, self.inner.desc) {
            (true, true) => quote! { #crate_location::types::OrderBoth },
            (true, false) => quote! { #crate_location::types::OrderAscOnly },
            (false, true) => quote! { #crate_location::types::OrderDescOnly },
            (false, false) => quote! {},
        })
    }
}
impl Parse for SyntaxSorterOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let for_token = input.parse::<Token![for]>()?;
        let target_ty = input.parse::<Type>()?;
        let impl_token = input.parse::<Token![impl]>()?;
        let puncs =
            syn::punctuated::Punctuated::<SyntaxSorterKind, Token![+]>::parse_separated_nonempty(
                input,
            )?;

        let mut result = Self {
            for_token,
            target_ty,
            impl_token,
            inner: ColumnSorterWith::default(),
        };
        for punc in puncs {
            match punc.inner {
                SorterKind::Asc => result.inner.asc = true,
                SorterKind::Desc => result.inner.desc = true,
                SorterKind::Values => result.inner.values = true,
            }
        }
        Ok(result)
    }
}

impl ToTokenWrapperSupport for SyntaxSorterOption {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        if self.inner.is_simple_order() {
            if self.inner.asc {
                tokens.extend(quote! {
                    Asc,
                });
            }
            if self.inner.desc {
                tokens.extend(quote! {
                    Desc,
                });
            }
            return;
        }
        let complex_order_rs = self.complex_order_rs(crate_location).unwrap();
        let order_variant = quote! { Order(#complex_order_rs), };
        let values_variant = match self.inner.values {
            true => {
                let ty = &self.target_ty;
                quote! { Values(::std::vec::Vec<#ty>), }
            }
            false => quote! {},
        };
        tokens.extend(quote! {
            #order_variant
            #values_variant
        })
    }
}

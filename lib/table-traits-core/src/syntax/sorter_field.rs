use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, token, Attribute, Field, Token, Type};

use crate::CrateLocation;

pub struct SyntaxSorterFields {
    pub brace_token: token::Brace,
    pub named: Punctuated<SyntaxSorterValiant, Token![,]>,
}

pub struct SyntaxSorterValiant {
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub fields: Field,
}

impl Parse for SyntaxSorterFields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace_token = braced!(content in input);
        let named = content.parse_terminated(SyntaxSorterValiant::parse, Token![,])?;
        Ok(Self { brace_token, named })
    }
}

impl Parse for SyntaxSorterValiant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let ident = input.parse()?;
        let fields = Field::parse_unnamed(input)?;
        Ok(Self {
            attrs,
            ident,
            fields,
        })
    }
}
impl SyntaxSorterValiant {
    pub fn to_token_stream(&self, target_ty: &Type, crate_location: CrateLocation) -> TokenStream {
        let attrs = &self.attrs;
        let ident = &self.ident;
        let field = &self.fields;
        quote! {
            #(#attrs)* #ident #field
        }
    }
}

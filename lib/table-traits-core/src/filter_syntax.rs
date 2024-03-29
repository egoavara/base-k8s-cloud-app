use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::filter_statement::FilterStatement;

#[derive(Debug)]
pub struct FilterSyntax {
    pub statements: Vec<FilterStatement>,
}

impl Parse for FilterSyntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();
        while !input.is_empty() {
            let statement = input.parse::<FilterStatement>()?;
            statements.push(statement);
        }

        Ok(FilterSyntax { statements })
    }
}

impl FilterSyntax {
    pub fn to_tokens(&self, crate_ident: &Ident) -> TokenStream {
        let FilterSyntax { statements, .. } = self;
        let statements = statements
            .iter()
            .map(|statement| statement.to_tokens(crate_ident));
        quote! {
            #(#statements)*
        }
    }
}

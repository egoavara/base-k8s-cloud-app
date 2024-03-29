use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::sorter_statement::SorterStatement;

#[derive(Debug)]
pub struct SorterSyntax {
    pub statements: Vec<SorterStatement>,
}

impl Parse for SorterSyntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();
        while !input.is_empty() {
            let statement = input.parse::<SorterStatement>()?;
            statements.push(statement);
        }

        Ok(SorterSyntax { statements })
    }
}

impl SorterSyntax {
    pub fn to_tokens(&self, crate_ident: &Ident) -> TokenStream {
        let SorterSyntax { statements, .. } = self;
        let statements = statements
            .iter()
            .map(|statement| statement.to_tokens(crate_ident));
        quote! {
            #(#statements)*
        }
    }
}

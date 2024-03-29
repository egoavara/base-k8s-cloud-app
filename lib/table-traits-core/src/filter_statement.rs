use proc_macro2::{Ident, TokenStream};
use quote::{quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{token, Attribute, Generics, Token, Type, Visibility, WhereClause};

use crate::filter_field::FilterFields;
use crate::filter_kind::FilterKind;

#[derive(Debug)]
pub struct FilterStatement {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub struct_token: Token![struct],
    pub ident: Ident,
    pub generics: Generics,
    pub for_token: Token![for],
    pub where_clause: Option<WhereClause>,
    pub target_ty: Type,
    pub fields: FilterFields,
    pub semi_token: Option<Token![;]>,
}

fn data_filter(
    input: ParseStream,
) -> syn::Result<(Option<WhereClause>, FilterFields, Option<Token![;]>)> {
    let where_clause = input.parse::<Option<WhereClause>>()?;
    let mut lookahead = input.lookahead1();

    if lookahead.peek(Token![impl]) {
        let impl_token = input.parse::<Token![impl]>()?;
        let fields = Punctuated::<FilterKind, Token![+]>::parse_separated_nonempty(input)?;
        lookahead = input.lookahead1();
        if lookahead.peek(Token![;]) {
            let semi = input.parse()?;
            Ok((
                where_clause,
                FilterFields::Implemented(impl_token, fields),
                Some(semi),
            ))
        } else {
            Err(lookahead.error())
        }
    } else if lookahead.peek(token::Brace) {
        let fields = input.parse()?;
        Ok((where_clause, FilterFields::Named(fields), None))
    } else if lookahead.peek(Token![;]) {
        let _ = input.parse::<Option<Token![;]>>()?;
        Err(lookahead.error())
    } else {
        Err(lookahead.error())
    }
}

impl Parse for FilterStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;
        let struct_token = input.parse::<Token![struct]>()?;
        let ident = input.parse::<Ident>()?;
        let generics = input.parse::<Generics>()?;
        let for_token = input.parse::<Token![for]>()?;
        let target_ty = input.parse::<Type>()?;
        let (where_clause, fields, semi_token) = data_filter(input)?;

        Ok(FilterStatement {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            for_token,
            target_ty,
            where_clause,
            fields,
            semi_token,
        })
    }
}

impl FilterStatement {
    pub fn to_tokens(&self, crate_ident: &Ident) -> TokenStream {
        let FilterStatement {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            for_token: _,
            where_clause,
            fields,
            target_ty,
            semi_token: _,
        } = self;
        let struct_fields = fields.to_struct_fields(crate_ident, target_ty).unwrap();
        let implemented_filters_exprs = fields.to_implemented_filters_exprs(crate_ident).unwrap();
        let activated_filters_exprs = fields.to_activated_filters_exprs(crate_ident).unwrap();
        let filter_value_match_arms = fields.to_filter_value_match_arms(crate_ident).unwrap();

        quote! {
            #(#attrs)*
            #vis #struct_token #ident #generics #where_clause {
                #(#struct_fields),*
            }

            impl ::#crate_ident::Filter for #ident{
                type Target = #target_ty;

                fn implemented_filters() -> ::std::vec::Vec<::#crate_ident::FilterKind>{
                    let mut result = ::std::vec::Vec::new();
                    #(#implemented_filters_exprs)*
                    result
                }

                fn activated_filters(&self) -> ::std::vec::Vec<::#crate_ident::FilterKind>{
                    let mut result = ::std::vec::Vec::new();
                    #(#activated_filters_exprs)*
                    result
                }

                fn filter_value(&self, kind: ::#crate_ident::FilterKind) -> ::#crate_ident::FilterValue<Self::Target>{
                    match kind{
                        #(#filter_value_match_arms)*
                        _ => ::#crate_ident::FilterValue::NotImplemented(kind),
                    }
                }
            }
        }
    }
}

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::{
    braced, Attribute, Generics, Token, Type, Visibility,
    WhereClause,
};

use crate::sorter_kind::SorterKind;
use crate::sorter_variant::SorterVariants;

#[derive(Debug)]
pub struct SorterStatement {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub enum_token: Token![enum],
    pub ident: Ident,
    pub generics: Generics,
    pub for_token: Token![for],
    pub where_clause: Option<WhereClause>,
    pub target_ty: Type,
    pub brace_token: Brace,
    pub variants: Punctuated<SorterVariants, Token![,]>,
}

fn data_sorter(
    input: ParseStream,
) -> syn::Result<(
    Option<WhereClause>,
    Brace,
    Punctuated<SorterVariants, Token![,]>,
)> {
    let where_clause = input.parse()?;

    let content;
    let brace = braced!(content in input);
    let variants =
        content.parse_terminated::<SorterVariants, _>(SorterVariants::parse, Token![,])?;
    Ok((where_clause, brace, variants))
}

impl Parse for SorterStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;
        let enum_token = input.parse::<Token![enum]>()?;
        let ident = input.parse::<Ident>()?;
        let generics = input.parse::<Generics>()?;
        let for_token = input.parse::<Token![for]>()?;
        let target_ty = input.parse::<Type>()?;

        let (where_clause, brace_token, variants) = data_sorter(input)?;
        Ok(SorterStatement {
            attrs,
            vis,
            enum_token,
            ident,
            generics,
            for_token,
            target_ty,
            where_clause,
            brace_token,
            variants,
        })
    }
}

impl SorterStatement {
    pub fn to_tokens(&self, crate_ident: &Ident) -> TokenStream {
        let SorterStatement {
            attrs,
            vis,
            enum_token,
            ident,
            generics,
            for_token: _,
            where_clause,
            target_ty,
            brace_token,
            variants,
        } = self;
        let none_variant = Some(SorterVariants::Implemented(
            Token![impl](Span::call_site()),
            SorterKind::None(Ident::new("none", Span::call_site())),
        ));
        let mut enum_define = quote! {
            #(#attrs)*
            #vis #enum_token #ident #generics #where_clause
        };
        brace_token.surround(&mut enum_define, |variants| {
            let mut is_first = true;
            for variant in self.variants.iter().chain(none_variant.iter()) {
                if is_first {
                    is_first = false;
                } else {
                    Token![,](Span::call_site()).to_tokens(variants);
                }
                variant
                    .to_variant(crate_ident, target_ty)
                    .to_tokens(variants);
                // variants.extend(variant.to_variant(crate_ident, target_ty));
            }
        });
        let implemented_sorter_exprs = variants
            .iter()
            .chain(none_variant.iter())
            .map(|v| v.implemented_sorter_expr(crate_ident))
            .collect::<Vec<_>>();
        let activated_sorter_match_arms = variants
            .iter()
            .chain(none_variant.iter())
            .map(|v| v.activated_sorter_match_arm(crate_ident))
            .collect::<Vec<_>>();
        let sorter_value_match_arms = variants
            .iter()
            .chain(none_variant.iter())
            .map(|v| v.sorter_value_match_arm(crate_ident))
            .collect::<Vec<_>>();

        let impl_fn = quote! {
            impl ::#crate_ident::Sorter for #ident{
                type Target = #target_ty;

                fn implemented() -> ::std::vec::Vec<::#crate_ident::SorterKind>{
                    let mut result = ::std::vec::Vec::new();
                    #(#implemented_sorter_exprs)*
                    result
                }

                fn activated(&self) -> ::#crate_ident::SorterKind{
                    match self{
                        #(#activated_sorter_match_arms)*
                    }
                }

                fn to_value(&self, kind: ::#crate_ident::SorterKind, value: Self::Target) -> ::#crate_ident::SorterValue<Self::Target>{

                    match kind{
                        #(#sorter_value_match_arms)*
                        _ => ::#crate_ident::SorterValue::NotImplemented(kind),
                    }
                }
            }
        };
        quote! {
            #enum_define
            #impl_fn
        }
    }
}

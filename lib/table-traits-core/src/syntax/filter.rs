use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Generics, Token, Type, Visibility, WhereClause};

use crate::derive_table::ColumnFilterWith;
use crate::enum_filter_kind::FilterKind;
use crate::syntax::filter_field::{SyntaxFieldType, SyntaxFilterFields};
use crate::syntax::filter_option::SyntaxFilterOption;
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

pub struct SyntaxFilterStatements {
    pub statements: Vec<SyntaxFilter>,
}

pub struct SyntaxFilter {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub struct_token: Token![struct],
    pub ident: Ident,
    pub generics: Generics,
    pub option: SyntaxFilterOption,
    pub where_clause: Option<WhereClause>,
    pub data_struct: SyntaxFilterFields,
}

impl SyntaxFilter {
    pub fn new_simple(
        attrs: Vec<Attribute>,
        vis: Visibility,
        ident: Ident,
        target_ty: Type,
        option: ColumnFilterWith,
    ) -> Self {
        Self {
            attrs,
            vis,
            struct_token: Default::default(),
            ident,
            generics: Default::default(),
            where_clause: Default::default(),
            option: SyntaxFilterOption {
                for_token: Default::default(),
                target_ty,
                impl_token: Default::default(),
                inner: option,
            },
            data_struct: SyntaxFilterFields {
                brace_token: Default::default(),
                named: Default::default(),
            },
        }
    }
}

impl Parse for SyntaxFilterStatements {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();
        while !input.is_empty() {
            let statement = input.parse::<SyntaxFilter>()?;
            statements.push(statement);
        }

        Ok(SyntaxFilterStatements { statements })
    }
}
impl Parse for SyntaxFilter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;
        let struct_token = input.parse::<Token![struct]>()?;
        let ident = input.parse::<Ident>()?;
        let generics = input.parse::<Generics>()?;
        let option = input.parse::<SyntaxFilterOption>()?;
        let where_clause = input.parse::<Option<WhereClause>>()?;
        let data_struct = input.parse::<SyntaxFilterFields>()?;
        Ok(SyntaxFilter {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            option,
            where_clause,
            data_struct,
        })
    }
}

impl ToTokenWrapperSupport for SyntaxFilterStatements {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        for x in &self.statements {
            x.to_tokens(tokens, crate_location);
        }
    }
}
impl SyntaxFilter {
    pub fn to_filter_kinds(&self) -> Vec<FilterKind> {
        let mut result = self.option.to_kinds();

        for x in &self.data_struct.named {
            match &x.ty {
                SyntaxFieldType::Impl(kind) => {
                    result.push(kind.inner);
                }
                _ => {}
            }
        }
        result
    }
}
impl SyntaxFilter {
    fn impl_filter(&self, crate_location: CrateLocation) -> TokenStream {
        let ident = &self.ident;
        let target_ty = &self.option.target_ty;

        let kinds = self.to_filter_kinds();
        let not_implemented = FilterKind::all()
            .into_iter()
            .filter(|x| !kinds.contains(x))
            .collect::<Vec<_>>();

        let impl_filters = kinds.iter().map(|x| {
            let kind = x.enum_value(Span::call_site());
            quote! {
                result.push(#crate_location::FilterKind::#kind)
            }
        });
        let match_arms_build_condition = kinds.iter().map(|x| {
            let field_ident = x.field_ident(Span::call_site());
            let enum_value =  x.enum_value(Span::call_site());
            match x {
                FilterKind::Eq => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_eq(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Ne => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_ne(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Gt => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_gt(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Lt => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_lt(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Gte => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_gte(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Lte => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_lte(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::In => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_in(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::NotIn => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_not_in(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Like => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_like(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::NLike => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_nlike(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Null => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .filter(|x|*x)
                        .map(|_| <Self::Target as #crate_location::private::FilterType>::expr_null(target_column.clone()))
                        .flatten()
                },
                FilterKind::NotNull => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .filter(|x|*x)
                        .map(|_| <Self::Target as #crate_location::private::FilterType>::expr_not_null(target_column.clone()))
                        .flatten()
                },
                FilterKind::Between => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_between(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::NBetween => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_nbetween(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Prefix => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_prefix(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::NPrefix => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_nprefix(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Suffix => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_suffix(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::NSuffix => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_nsuffix(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Contain => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_contain(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::NContain => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_ncontain(x, target_column.clone()))
                        .flatten()
                },
                FilterKind::Regex => quote!{
                    #crate_location::FilterKind::#enum_value => self.#field_ident
                        .as_ref()
                        .cloned()
                        .map(|x| <Self::Target as #crate_location::private::FilterType>::expr_regex(x, target_column.clone()))
                        .flatten()
                },
            }
        });
        let match_arms_not_implemented = not_implemented.iter().map(|x| {
            let enum_value = x.enum_value(Span::call_site());
            quote! {
                #crate_location::FilterKind::#enum_value => None
            }
        });

        quote! {
            impl #crate_location::Filter for #ident {

                type Target = #target_ty;

                fn implemented_filters() -> Vec<#crate_location::FilterKind> {
                    let mut result = ::std::vec::Vec::new();
                    #(#impl_filters;)*
                    result
                }
                fn build_condition(&self, filter_kind: #crate_location::FilterKind, target_column: impl ::sea_query::IntoColumnRef + ::core::clone::Clone) -> ::core::option::Option<::sea_query::Condition>{
                    match filter_kind{
                        #(#match_arms_build_condition,)*
                        #(#match_arms_not_implemented,)*
                    }
                        .map(|x|x.into_condition())
                }
            }
        }
    }
}

impl ToTokenWrapperSupport for SyntaxFilter {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let struct_token = &self.struct_token;
        let ident = &self.ident;
        let generics = &self.generics;
        let where_clause = &self.where_clause;

        let option_rs = &self.option.to_token_stream(crate_location);
        let fields = self
            .data_struct
            .named
            .iter()
            .map(|x| x.to_token_stream(&self.option.target_ty, crate_location));

        let impl_filter_rs = self.impl_filter(crate_location);
        tokens.extend(quote! {
            #(#attrs)*
            #vis #struct_token #ident #generics #where_clause{
                #option_rs
                #(#fields)*
            }

            #impl_filter_rs
        });
    }
}

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Generics, Token, Type, Visibility, WhereClause};

use crate::derive_table::ColumnSorterWith;
use crate::syntax::{SyntaxSorterFields, SyntaxSorterOption};
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

pub struct SyntaxSorterStatements {
    pub statements: Vec<SyntaxSorter>,
}
pub struct SyntaxSorter {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub enum_token: Token![enum],
    pub ident: Ident,
    pub generics: Generics,
    pub option: SyntaxSorterOption,
    pub where_clause: Option<WhereClause>,
    pub data_enum: SyntaxSorterFields,
}

impl SyntaxSorter {
    pub fn new_simple(
        attrs: Vec<Attribute>,
        vis: Visibility,
        ident: Ident,
        target_ty: Type,
        option: ColumnSorterWith,
    ) -> Self {
        Self {
            attrs,
            vis,
            enum_token: Default::default(),
            ident,
            generics: Default::default(),
            where_clause: Default::default(),
            option: SyntaxSorterOption {
                for_token: Default::default(),
                target_ty,
                impl_token: Default::default(),
                inner: option,
            },
            data_enum: SyntaxSorterFields {
                brace_token: Default::default(),
                named: Default::default(),
            },
        }
    }
}

impl Parse for SyntaxSorterStatements {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();
        while !input.is_empty() {
            let statement = input.parse::<SyntaxSorter>()?;
            statements.push(statement);
        }

        Ok(SyntaxSorterStatements { statements })
    }
}
impl Parse for SyntaxSorter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;
        let enum_token = input.parse::<Token![enum]>()?;
        let ident = input.parse::<Ident>()?;
        let generics = input.parse::<Generics>()?;
        let option = input.parse::<SyntaxSorterOption>()?;
        let where_clause = input.parse::<Option<WhereClause>>()?;
        let data_struct = input.parse::<SyntaxSorterFields>()?;
        Ok(SyntaxSorter {
            attrs,
            vis,
            enum_token,
            ident,
            generics,
            option,
            where_clause,
            data_enum: data_struct,
        })
    }
}

impl ToTokenWrapperSupport for SyntaxSorterStatements {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        for x in &self.statements {
            x.to_tokens(tokens, crate_location);
        }
    }
}
impl SyntaxSorter {
    fn impl_filter(&self, crate_location: CrateLocation) -> TokenStream {
        if self.option.inner.is_simple_order() {
            self.impl_filter_simple(crate_location)
        } else {
            self.impl_filter_complex(crate_location)
        }
    }
    fn impl_filter_simple(&self, crate_location: CrateLocation) -> TokenStream {
        let ident = &self.ident;
        let target_ty = &self.option.target_ty;

        let mut order_rs = TokenStream::new();
        if self.option.inner.asc {
            order_rs.extend(quote! {
                #ident::Asc => (
                    ::sea_query::SimpleExpr::Column(target_column.clone().into_column_ref()),
                    ::sea_query::Order::Asc,
                    ::core::option::Option::None,
                ),
            });
        }
        if self.option.inner.desc {
            order_rs.extend(quote! {
                #ident::Desc => (
                    ::sea_query::SimpleExpr::Column(target_column.clone().into_column_ref()),
                    ::sea_query::Order::Desc,
                    ::core::option::Option::None,
                ),
            });
        }

        let mut after_rs = TokenStream::new();
        if self.option.inner.asc {
            after_rs.extend(quote! {
                #ident::Asc => ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column.clone()).gt(value)),
            });
        }
        if self.option.inner.desc {
            after_rs.extend(quote! {
                #ident::Desc => ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column.clone()).lt(value)),
            });
        }

        let mut before_rs = TokenStream::new();
        if self.option.inner.asc {
            before_rs.extend(quote! {
                #ident::Asc => ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column.clone()).lt(value)),
            });
        }
        if self.option.inner.desc {
            before_rs.extend(quote! {
                #ident::Desc => ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column.clone()).gt(value)),
            });
        }

        quote! {
            impl #crate_location::Sorter for #ident {

                type Target = #target_ty;

                fn build_order(&self, target_column: impl ::sea_query::IntoColumnRef + ::core::clone::Clone)
                    -> (::sea_query::SimpleExpr, ::sea_query::Order, ::core::option::Option<::sea_query::NullOrdering>){
                    match self{
                        #order_rs
                    }
                }

                fn build_equal(
                    &self,
                    value: Self::Target,
                    target_column: impl ::sea_query::IntoColumnRef + ::core::clone::Clone,
                ) -> ::sea_query::Condition{
                    ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column).eq(value))
                }

                fn build_after(
                    &self,
                    value: Self::Target,
                    target_column: impl ::sea_query::IntoColumnRef + ::core::clone::Clone,
                ) -> ::sea_query::Condition{
                    match self{
                        #after_rs
                    }
                }

                fn build_before(
                    &self,
                    value: Self::Target,
                    target_column: impl ::sea_query::IntoColumnRef + ::core::clone::Clone,
                ) -> ::sea_query::Condition{
                    match self{
                        #before_rs
                    }
                }
            }
        }
    }
    fn impl_filter_complex(&self, crate_location: CrateLocation) -> TokenStream {
        let ident = &self.ident;
        let target_ty = &self.option.target_ty;

        let complex_order_rs = self.option.complex_order_rs(crate_location).unwrap();

        let order_rs = self.impl_filter_complex_order(ident, complex_order_rs);
        let after_rs = self.impl_filter_complex_after(crate_location);
        let before_rs = self.impl_filter_complex_before(crate_location);

        quote! {
            impl #crate_location::Sorter for #ident {

                type Target = #target_ty;

                fn build_order(&self, target_column: impl ::sea_query::IntoColumnRef + ::core::clone::Clone)
                    -> (::sea_query::SimpleExpr, ::sea_query::Order, ::core::option::Option<::sea_query::NullOrdering>){
                    match self{
                        #order_rs
                    }
                }

                fn build_equal(
                    &self,
                    value: Self::Target,
                    target_column: impl ::sea_query::IntoColumnRef + ::core::clone::Clone,
                ) -> ::sea_query::Condition{
                    ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column).eq(value))
                }

                fn build_after(
                    &self,
                    value: Self::Target,
                    target_column: impl ::sea_query::IntoColumnRef + ::core::clone::Clone,
                ) -> ::sea_query::Condition{
                    match self{
                        #after_rs
                    }
                }

                fn build_before(
                    &self,
                    value: Self::Target,
                    target_column: impl ::sea_query::IntoColumnRef + ::core::clone::Clone,
                ) -> ::sea_query::Condition{
                    match self{
                        #before_rs
                    }
                }
            }
        }
    }

    fn impl_filter_complex_order(
        &self,
        ident: &Ident,
        complex_order_rs: TokenStream,
    ) -> TokenStream {
        let mut order_rs = TokenStream::new();
        if self.option.inner.asc {
            order_rs.extend(quote! {
                #ident::Order(#complex_order_rs::Asc) => (
                    ::sea_query::SimpleExpr::Column(target_column.clone().into_column_ref()),
                    ::sea_query::Order::Asc,
                    ::core::option::Option::None,
                ),
            });
        }
        if self.option.inner.desc {
            order_rs.extend(quote! {
                #ident::Order(#complex_order_rs::Desc) => (
                    ::sea_query::SimpleExpr::Column(target_column.clone().into_column_ref()),
                    ::sea_query::Order::Desc,
                    ::core::option::Option::None,
                ),
            });
        }
        if self.option.inner.values {
            order_rs.extend(quote! {
                #ident::Values(values) => (
                    ::sea_query::SimpleExpr::Column(target_column.clone().into_column_ref()),
                    ::sea_query::Order::Field(::sea_query::Values(values.iter().map(|x| ::sea_query::Value::from(x)).collect())),
                    ::core::option::Option::None,
                ),
            });
        }
        order_rs
    }
    fn impl_filter_complex_after(&self, crate_location: CrateLocation) -> TokenStream {
        let ident = &self.ident;
        let complex_order_rs = self.option.complex_order_rs(crate_location).unwrap();
        let mut condition_rs = TokenStream::new();
        if self.option.inner.asc {
            condition_rs.extend(quote! {
                #ident::Order(#complex_order_rs::Asc) => ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column.clone()).gt(value)),
            });
        }
        if self.option.inner.desc {
            condition_rs.extend(quote! {
                #ident::Order(#complex_order_rs::Desc) => ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column.clone()).lt(value)),
            });
        }
        if self.option.inner.values {
            condition_rs.extend(quote! {
                #ident::Values(values) => {
                    let skipped = values.iter()
                        .skip_while(|&x|x.ne(&value))
                        .skip(1)
                        .map(|x| ::sea_query::Value::from(x))
                        .collect::<Vec<_>>();
                    ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column.clone()).is_in(skipped))
                },
            });
        }
        condition_rs
    }
    fn impl_filter_complex_before(&self, crate_location: CrateLocation) -> TokenStream {
        let ident = &self.ident;
        let complex_order_rs = self.option.complex_order_rs(crate_location).unwrap();
        let mut condition_rs = TokenStream::new();
        if self.option.inner.asc {
            condition_rs.extend(quote! {
                #ident::Order(#complex_order_rs::Asc) => ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column.clone()).lt(value)),
            });
        }
        if self.option.inner.desc {
            condition_rs.extend(quote! {
                #ident::Order(#complex_order_rs::Desc) => ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column.clone()).gt(value)),
            });
        }
        if self.option.inner.values {
            condition_rs.extend(quote! {
                #ident::Values(values) => {
                    let skipped = values.iter()
                        .rev()
                        .skip_while(|&x|x.ne(&value))
                        .skip(1)
                        .map(|x| ::sea_query::Value::from(x))
                        .collect::<Vec<_>>();
                    ::sea_query::IntoCondition::into_condition(::sea_query::Expr::col(target_column.clone()).is_in(skipped))
                },
            });
        }
        condition_rs
    }
}

impl ToTokenWrapperSupport for SyntaxSorter {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let enum_token = &self.enum_token;
        let ident = &self.ident;
        let generics = &self.generics;
        let where_clause = &self.where_clause;

        let option_rs = &self.option.to_token_stream(crate_location);
        let fields = self
            .data_enum
            .named
            .iter()
            .map(|x| x.to_token_stream(&self.option.target_ty, crate_location));

        let impl_filter_rs = self.impl_filter(crate_location);
        tokens.extend(quote! {
            #(#attrs)*
            #vis #enum_token #ident #generics #where_clause{
                #option_rs
                #(#fields)*
            }

            #impl_filter_rs
        });
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sorter_() {}
}

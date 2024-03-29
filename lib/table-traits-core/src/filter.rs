use proc_macro2::{Ident, TokenStream};


use crate::filter_syntax::FilterSyntax;

pub fn filter(crate_ident: &'static str, input: TokenStream) -> TokenStream {
    let crate_ident = Ident::new(crate_ident, proc_macro2::Span::call_site());
    let ast: FilterSyntax = syn::parse2(input).unwrap();
    ast.to_tokens(&crate_ident)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use prettyplease::unparse;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn tuple_style_string_filter() {
        let input: TokenStream = parse_quote! {
            pub struct Filter for String impl eq + ne + in + between;
        };
        let expect = quote! {

            pub struct Filter {
                eq: ::core::option::Option<String>,
                ne: ::core::option::Option<String>,
                r#in: ::std::vec::Vec<String>,
                between: ::core::option::Option<::table_trait::Range<String> >
            }

            impl ::table_trait::Filter for Filter {
                type Target = String;
                fn implemented_filters() -> ::std::vec::Vec<::table_trait::FilterKind> {
                    let mut result = ::std::vec::Vec::new();
                    result.push(::table_trait::FilterKind::Eq);
                    result.push(::table_trait::FilterKind::Ne);
                    result.push(::table_trait::FilterKind::In);
                    result.push(::table_trait::FilterKind::Between);
                    result
                }
                fn activated_filters(&self) -> ::std::vec::Vec<::table_trait::FilterKind> {
                    let mut result = ::std::vec::Vec::new();
                    if self.eq.is_some() {
                        result.push(::table_trait::FilterKind::Eq);
                    }
                    if self.ne.is_some() {
                        result.push(::table_trait::FilterKind::Ne);
                    }
                    if !self.r#in.is_empty() {
                        result.push(::table_trait::FilterKind::In);
                    }
                    if self.between.is_some() {
                        result.push(::table_trait::FilterKind::Between);
                    }
                    result
                }
                fn filter_value(&self, kind: ::table_trait::FilterKind) -> ::table_trait::FilterValue<Self::Target> {
                    match kind {
                        ::table_trait::FilterKind::Eq => match &self.eq {
                            ::core::option::Option::Some(value) => {
                                ::table_trait::FilterValue::Eq(value.clone())
                            }
                            ::core::option::Option::None => ::table_trait::FilterValue::None,
                        }
                        ::table_trait::FilterKind::Ne => match &self.ne {
                            ::core::option::Option::Some(value) => {
                                ::table_trait::FilterValue::Ne(value.clone())
                            }
                            ::core::option::Option::None => ::table_trait::FilterValue::None,
                        }
                        ::table_trait::FilterKind::In => {
                            if self.r#in.len() > 0 {
                                ::table_trait::FilterValue::In(self.r#in.clone())
                            } else {
                                ::table_trait::FilterValue::None
                            }
                        }
                        ::table_trait::FilterKind::Between => match &self.between {
                            ::core::option::Option::Some(value) => {
                                ::table_trait::FilterValue::Between(value.min.clone(), value.max.clone())
                            }
                            ::core::option::Option::None => ::table_trait::FilterValue::None,
                        }
                        _ => ::table_trait::FilterValue::NotImplemented(kind),
                    }
                }
            }
        };
        let actual = filter("table_trait", input);
        let expect_syn: syn::File = syn::parse2(expect).unwrap();
        let actual_syn: syn::File = syn::parse2(actual).unwrap();
        let pretty_actual = unparse(&actual_syn);
        let pretty_expect = unparse(&expect_syn);
        assert_eq!(pretty_actual, pretty_expect);
    }
}

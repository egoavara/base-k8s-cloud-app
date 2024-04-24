use crate::CrateLocation;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Token, Type};

use crate::enum_filter_kind::FilterKind;
use crate::utils::ToTokenWrapperSupport;

mod kw {
    syn::custom_keyword!(eq);
    syn::custom_keyword!(ne);
    syn::custom_keyword!(gt);
    syn::custom_keyword!(lt);
    syn::custom_keyword!(gte);
    syn::custom_keyword!(lte);
    syn::custom_keyword!(not_in);
    syn::custom_keyword!(like);
    syn::custom_keyword!(nlike);
    syn::custom_keyword!(null);
    syn::custom_keyword!(not_null);
    syn::custom_keyword!(between);
    syn::custom_keyword!(nbetween);
    syn::custom_keyword!(prefix);
    syn::custom_keyword!(nprefix);
    syn::custom_keyword!(suffix);
    syn::custom_keyword!(nsuffix);
    syn::custom_keyword!(contains);
    syn::custom_keyword!(ncontains);
    syn::custom_keyword!(regex);
}

pub struct SyntaxFilterKind {
    pub inner: FilterKind,
    pub span: Span,
}

impl Parse for SyntaxFilterKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![in]) {
            input.parse::<Token![in]>()?;
            Ok(Self {
                inner: FilterKind::In,
                span: input.span(),
            })
        } else if lookahead.peek(kw::eq) {
            input.parse::<kw::eq>()?;
            Ok(Self {
                inner: FilterKind::Eq,
                span: input.span(),
            })
        } else if lookahead.peek(kw::ne) {
            input.parse::<kw::ne>()?;
            Ok(Self {
                inner: FilterKind::Ne,
                span: input.span(),
            })
        } else if lookahead.peek(kw::gt) {
            input.parse::<kw::gt>()?;
            Ok(Self {
                inner: FilterKind::Gt,
                span: input.span(),
            })
        } else if lookahead.peek(kw::lt) {
            input.parse::<kw::lt>()?;
            Ok(Self {
                inner: FilterKind::Lt,
                span: input.span(),
            })
        } else if lookahead.peek(kw::gte) {
            input.parse::<kw::gte>()?;
            Ok(Self {
                inner: FilterKind::Gte,
                span: input.span(),
            })
        } else if lookahead.peek(kw::lte) {
            input.parse::<kw::lte>()?;
            Ok(Self {
                inner: FilterKind::Lte,
                span: input.span(),
            })
        } else if lookahead.peek(kw::not_in) {
            input.parse::<kw::not_in>()?;
            Ok(Self {
                inner: FilterKind::NotIn,
                span: input.span(),
            })
        } else if lookahead.peek(kw::like) {
            input.parse::<kw::like>()?;
            Ok(Self {
                inner: FilterKind::Like,
                span: input.span(),
            })
        } else if lookahead.peek(kw::nlike) {
            input.parse::<kw::nlike>()?;
            Ok(Self {
                inner: FilterKind::NLike,
                span: input.span(),
            })
        } else if lookahead.peek(kw::null) {
            input.parse::<kw::null>()?;
            Ok(Self {
                inner: FilterKind::Null,
                span: input.span(),
            })
        } else if lookahead.peek(kw::not_null) {
            input.parse::<kw::not_null>()?;
            Ok(Self {
                inner: FilterKind::NotNull,
                span: input.span(),
            })
        } else if lookahead.peek(kw::between) {
            input.parse::<kw::between>()?;
            Ok(Self {
                inner: FilterKind::Between,
                span: input.span(),
            })
        } else if lookahead.peek(kw::nbetween) {
            input.parse::<kw::nbetween>()?;
            Ok(Self {
                inner: FilterKind::NBetween,
                span: input.span(),
            })
        } else if lookahead.peek(kw::prefix) {
            input.parse::<kw::prefix>()?;
            Ok(Self {
                inner: FilterKind::Prefix,
                span: input.span(),
            })
        } else if lookahead.peek(kw::nprefix) {
            input.parse::<kw::nprefix>()?;
            Ok(Self {
                inner: FilterKind::NPrefix,
                span: input.span(),
            })
        } else if lookahead.peek(kw::suffix) {
            input.parse::<kw::suffix>()?;
            Ok(Self {
                inner: FilterKind::Suffix,
                span: input.span(),
            })
        } else if lookahead.peek(kw::nsuffix) {
            input.parse::<kw::nsuffix>()?;
            Ok(Self {
                inner: FilterKind::NSuffix,
                span: input.span(),
            })
        } else if lookahead.peek(kw::contains) {
            input.parse::<kw::contains>()?;
            Ok(Self {
                inner: FilterKind::Contain,
                span: input.span(),
            })
        } else if lookahead.peek(kw::ncontains) {
            input.parse::<kw::ncontains>()?;
            Ok(Self {
                inner: FilterKind::NContain,
                span: input.span(),
            })
        } else if lookahead.peek(kw::regex) {
            input.parse::<kw::regex>()?;
            Ok(Self {
                inner: FilterKind::Regex,
                span: input.span(),
            })
        } else {
            return Err(lookahead.error());
        }
    }
}

impl SyntaxFilterKind {
    pub fn to_token_stream(&self, target_ty: &Type, crate_location: CrateLocation) -> TokenStream {
        match self.inner {
            FilterKind::Eq => {
                quote! {::core::option::Option<#target_ty>}
            }
            FilterKind::Ne => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::Gt => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::Lt => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::Gte => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::Lte => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::In => {
                quote! {::core::option::Option<::std::vec::Vec<#target_ty>>}
            }

            FilterKind::NotIn => {
                quote! {::core::option::Option<::std::vec::Vec<#target_ty>>}
            }

            FilterKind::Like => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::NLike => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::Null => {
                quote! {::core::option::Option<bool>}
            }

            FilterKind::NotNull => {
                quote! {::core::option::Option<bool>}
            }

            FilterKind::Between => {
                quote! {::core::option::Option<#crate_location::Range<#target_ty>>}
            }

            FilterKind::NBetween => {
                quote! {::core::option::Option<#crate_location::Range<#target_ty>>}
            }

            FilterKind::Prefix => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::NPrefix => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::Suffix => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::NSuffix => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::Contain => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::NContain => {
                quote! {::core::option::Option<#target_ty>}
            }

            FilterKind::Regex => {
                quote! {::core::option::Option<#target_ty>}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::enum_filter_kind::FilterKind;
    use crate::syntax::filter_kind::SyntaxFilterKind;
    use quote::quote;

    #[test]
    fn test_parse() {
        let input = quote! {
            eq
        };
        let actual: SyntaxFilterKind = syn::parse2(input).unwrap();
        assert_eq!(actual.inner, FilterKind::Eq);
    }
    #[test]
    fn test_parse_in() {
        let input = quote! {
            in
        };
        let actual: SyntaxFilterKind = syn::parse2(input).unwrap();
        assert_eq!(actual.inner, FilterKind::In);
    }
}

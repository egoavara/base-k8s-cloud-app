use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Type;

use crate::enum_sorter_kind::SorterKind;
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

mod kw {
    syn::custom_keyword!(asc);
    syn::custom_keyword!(desc);
    syn::custom_keyword!(values);
}

pub struct SyntaxSorterKind {
    pub inner: SorterKind,
    pub span: Span,
}

impl Parse for SyntaxSorterKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::asc) {
            input.parse::<kw::asc>()?;
            Ok(Self {
                inner: SorterKind::Asc,
                span: input.span(),
            })
        } else if lookahead.peek(kw::desc) {
            input.parse::<kw::desc>()?;
            Ok(Self {
                inner: SorterKind::Desc,
                span: input.span(),
            })
        } else if lookahead.peek(kw::values) {
            input.parse::<kw::values>()?;
            Ok(Self {
                inner: SorterKind::Values,
                span: input.span(),
            })
        } else {
            return Err(lookahead.error());
        }
    }
}

#[cfg(test)]
mod test {
    use quote::quote;

    use crate::enum_filter_kind::FilterKind;
    use crate::syntax::filter_kind::SyntaxFilterKind;

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

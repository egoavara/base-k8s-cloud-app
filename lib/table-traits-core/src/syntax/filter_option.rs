use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Token, Type};

use crate::derive_table::ColumnFilterWith;
use crate::enum_filter_kind::FilterKind;
use crate::syntax::filter_kind::SyntaxFilterKind;
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

pub struct SyntaxFilterOption {
    pub for_token: Token![for],
    pub target_ty: Type,
    pub impl_token: Token![impl],
    pub inner: ColumnFilterWith,
}
impl SyntaxFilterOption {
    pub fn to_kinds(&self) -> Vec<FilterKind> {
        let mut kinds = Vec::<FilterKind>::new();
        if self.inner.eq {
            kinds.push(FilterKind::Eq);
        }
        if self.inner.ne {
            kinds.push(FilterKind::Ne);
        }
        if self.inner.gt {
            kinds.push(FilterKind::Gt);
        }
        if self.inner.lt {
            kinds.push(FilterKind::Lt);
        }
        if self.inner.gte {
            kinds.push(FilterKind::Gte);
        }
        if self.inner.lte {
            kinds.push(FilterKind::Lte);
        }
        if self.inner.r#in {
            kinds.push(FilterKind::In);
        }
        if self.inner.not_in {
            kinds.push(FilterKind::NotIn);
        }
        if self.inner.like {
            kinds.push(FilterKind::Like);
        }
        if self.inner.nlike {
            kinds.push(FilterKind::NLike);
        }
        if self.inner.null {
            kinds.push(FilterKind::Null);
        }
        if self.inner.not_null {
            kinds.push(FilterKind::NotNull);
        }
        if self.inner.between {
            kinds.push(FilterKind::Between);
        }
        if self.inner.nbetween {
            kinds.push(FilterKind::NBetween);
        }
        if self.inner.prefix {
            kinds.push(FilterKind::Prefix);
        }
        if self.inner.nprefix {
            kinds.push(FilterKind::NPrefix);
        }
        if self.inner.suffix {
            kinds.push(FilterKind::Suffix);
        }
        if self.inner.nsuffix {
            kinds.push(FilterKind::NSuffix);
        }
        if self.inner.contains {
            kinds.push(FilterKind::Contain);
        }
        if self.inner.ncontains {
            kinds.push(FilterKind::NContain);
        }
        if self.inner.regex {
            kinds.push(FilterKind::Regex);
        }
        kinds
    }
}
impl Parse for SyntaxFilterOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let for_token = input.parse::<Token![for]>()?;
        let target_ty = input.parse::<Type>()?;
        let impl_token = input.parse::<Token![impl]>()?;
        let puncs =
            syn::punctuated::Punctuated::<SyntaxFilterKind, Token![+]>::parse_separated_nonempty(
                input,
            )?;

        let mut result = Self {
            for_token,
            target_ty,
            impl_token,
            inner: ColumnFilterWith::default(),
        };
        for punc in puncs {
            match punc.inner {
                FilterKind::Eq => result.inner.eq = true,
                FilterKind::Ne => result.inner.ne = true,
                FilterKind::Gt => result.inner.gt = true,
                FilterKind::Lt => result.inner.lt = true,
                FilterKind::Gte => result.inner.gte = true,
                FilterKind::Lte => result.inner.lte = true,
                FilterKind::In => result.inner.r#in = true,
                FilterKind::NotIn => result.inner.not_in = true,
                FilterKind::Like => result.inner.like = true,
                FilterKind::NLike => result.inner.nlike = true,
                FilterKind::Null => result.inner.null = true,
                FilterKind::NotNull => result.inner.not_null = true,
                FilterKind::Between => result.inner.between = true,
                FilterKind::NBetween => result.inner.nbetween = true,
                FilterKind::Prefix => result.inner.prefix = true,
                FilterKind::NPrefix => result.inner.nprefix = true,
                FilterKind::Suffix => result.inner.suffix = true,
                FilterKind::NSuffix => result.inner.nsuffix = true,
                FilterKind::Contain => result.inner.contains = true,
                FilterKind::NContain => result.inner.ncontains = true,
                FilterKind::Regex => result.inner.regex = true,
            }
        }
        Ok(result)
    }
}

impl ToTokenWrapperSupport for SyntaxFilterOption {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        let kinds = self
            .to_kinds()
            .into_iter()
            .map(|x| {
                let ident = x.field_ident(Span::call_site());
                let ty = SyntaxFilterKind {
                    inner: x,
                    span: Span::call_site(),
                }
                .to_token_stream(&self.target_ty, crate_location);

                quote! { pub #ident: #ty}
            })
            .collect::<Vec<_>>();
        tokens.extend(quote! {
            #(#kinds,)*
        })
    }
}

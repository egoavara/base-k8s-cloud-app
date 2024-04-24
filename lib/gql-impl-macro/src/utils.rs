use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::CrateLocation;

#[derive(Debug, FromMeta)]
pub struct Empty {}

pub trait ToTokenWrapperSupport {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation);
    fn to_token_stream(&self, crate_location: CrateLocation) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens, crate_location);
        tokens
    }
}

pub struct ToTokenWrapper<T> {
    pub(crate) inner: T,
    pub(crate) crate_location: CrateLocation,
}

impl<T> ToTokenWrapper<T> {
    pub fn new(crate_location: CrateLocation, inner: T) -> Self {
        Self {
            inner,
            crate_location,
        }
    }
}

impl<T: ToTokenWrapperSupport> ToTokens for ToTokenWrapper<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.inner.to_tokens(tokens, self.crate_location);
    }
}

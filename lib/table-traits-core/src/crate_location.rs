use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrateLocation {
    InCrate,
    OtherSubCrate,
    Outside,
}

impl ToTokens for CrateLocation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CrateLocation::InCrate => {
                tokens.extend(quote! {crate});
            }
            CrateLocation::OtherSubCrate => {
                tokens.extend(quote! {::table_traits_impl});
            }
            CrateLocation::Outside => {
                tokens.extend(quote! {::table_traits});
            }
        }
    }
}

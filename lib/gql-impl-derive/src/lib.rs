use proc_macro::TokenStream;

use gql_impl_macro::CrateLocation;

#[proc_macro_derive(Table, attributes(table, column))]
pub fn table(input: TokenStream) -> TokenStream {
    gql_impl_macro::table(CrateLocation::Outside, input.into()).into()
}

#[proc_macro]
pub fn filter(input: TokenStream) -> TokenStream {
    gql_impl_macro::filter(CrateLocation::Outside, input.into()).into()
}

#[proc_macro]
pub fn filter_internal(input: TokenStream) -> TokenStream {
    gql_impl_macro::filter(CrateLocation::OtherSubCrate, input.into()).into()
}

#[proc_macro]
pub fn filter_crate(input: TokenStream) -> TokenStream {
    gql_impl_macro::filter(CrateLocation::InCrate, input.into()).into()
}

#[proc_macro]
pub fn sorter(input: TokenStream) -> TokenStream {
    gql_impl_macro::sorter(CrateLocation::Outside, input.into()).into()
}

#[proc_macro]
pub fn sorter_internal(input: TokenStream) -> TokenStream {
    gql_impl_macro::sorter(CrateLocation::OtherSubCrate, input.into()).into()
}

#[proc_macro]
pub fn sorter_crate(input: TokenStream) -> TokenStream {
    gql_impl_macro::sorter(CrateLocation::InCrate, input.into()).into()
}

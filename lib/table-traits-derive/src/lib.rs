use proc_macro::TokenStream;

use table_traits_core::CrateLocation;

#[proc_macro_derive(Table, attributes(table, column))]
pub fn table(input: TokenStream) -> TokenStream {
    table_traits_core::table(CrateLocation::Outside, input.into()).into()
}

#[proc_macro]
pub fn filter(input: TokenStream) -> TokenStream {
    table_traits_core::filter(CrateLocation::Outside, input.into()).into()
}

#[proc_macro]
pub fn filter_internal(input: TokenStream) -> TokenStream {
    table_traits_core::filter(CrateLocation::OtherSubCrate, input.into()).into()
}

#[proc_macro]
pub fn filter_crate(input: TokenStream) -> TokenStream {
    table_traits_core::filter(CrateLocation::InCrate, input.into()).into()
}

#[proc_macro]
pub fn sorter(input: TokenStream) -> TokenStream {
    table_traits_core::sorter(CrateLocation::Outside, input.into()).into()
}

#[proc_macro]
pub fn sorter_internal(input: TokenStream) -> TokenStream {
    table_traits_core::sorter(CrateLocation::OtherSubCrate, input.into()).into()
}

#[proc_macro]
pub fn sorter_crate(input: TokenStream) -> TokenStream {
    table_traits_core::sorter(CrateLocation::InCrate, input.into()).into()
}

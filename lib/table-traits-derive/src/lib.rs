use proc_macro::TokenStream;

#[proc_macro_derive(Table, attributes(table))]
pub fn table(input: TokenStream) -> TokenStream {
    table_traits_core::table("table_traits", input.into()).into()
}

#[proc_macro]
pub fn filter(input: TokenStream) -> TokenStream {
    table_traits_core::filter("table_traits", input.into()).into()
}

#[proc_macro]
pub fn filter_internal(input: TokenStream) -> TokenStream {
    table_traits_core::filter("table_traits_impl", input.into()).into()
}

#[proc_macro]
pub fn sorter(input: TokenStream) -> TokenStream {
    table_traits_core::sorter("table_traits", input.into()).into()
}

#[proc_macro]
pub fn sorter_internal(input: TokenStream) -> TokenStream {
    table_traits_core::sorter("table_traits_impl", input.into()).into()
}

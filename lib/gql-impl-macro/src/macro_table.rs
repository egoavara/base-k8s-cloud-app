use darling::FromDeriveInput;
use proc_macro2::TokenStream;

use crate::crate_location::CrateLocation;
use crate::derive_table::Table;
use crate::sorter;
use crate::utils::ToTokenWrapperSupport;

pub fn table(crate_location: CrateLocation, input: TokenStream) -> TokenStream {
    let source = syn::parse2(input).unwrap();
    let actual = Table::from_derive_input(&source).unwrap();
    actual.to_token_stream(crate_location)
}
#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use prettyplease::unparse;
    use quote::quote;

    use super::*;

    #[test]
    fn tuple_table_example() {
        let input = quote! {
            #[derive(Table)]
            pub struct Test{
                #[column(id)]
                pub id: Uuid,
                #[column(filter, sorter)]
                pub name: String,
                #[column(filter)]
                pub age: u16,
                #[column(sorter)]
                pub phone: String,
            }
        };
        let expect = quote! {
            pub struct DefaultUuidSorter {

            }
        };
        let actual = table(CrateLocation::Outside, input);
        let expect_syn: syn::File = syn::parse2(expect).unwrap();
        let actual_syn: syn::File = syn::parse2(actual).unwrap();
        let pretty_actual = unparse(&actual_syn);
        let pretty_expect = unparse(&expect_syn);
        println!("{}", pretty_actual);
        assert_eq!(pretty_actual, pretty_expect);
    }
}

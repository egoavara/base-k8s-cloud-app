use proc_macro2::TokenStream;

use crate::crate_location::CrateLocation;
use crate::syntax::{
    SyntaxFilterStatements, SyntaxSorter, SyntaxSorterFields, SyntaxSorterStatements,
};
use crate::utils::ToTokenWrapperSupport;

pub fn sorter(crate_location: CrateLocation, input: TokenStream) -> TokenStream {
    let ast: SyntaxSorterStatements = syn::parse2(input).unwrap();
    ast.to_token_stream(crate_location)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use prettyplease::unparse;
    use quote::quote;

    use super::*;

    #[test]
    fn tuple_style_string_filter() {
        let input = quote! {
            pub enum DefaultStringSorter for String impl asc + desc + values{


            }
        };
        let expect = quote! {
            pub struct DefaultUuidSorter {

            }
        };
        let actual = sorter(CrateLocation::Outside, input);
        let expect_syn: syn::File = syn::parse2(expect).unwrap();
        let actual_syn: syn::File = syn::parse2(actual).unwrap();
        let pretty_actual = unparse(&actual_syn);
        let pretty_expect = unparse(&expect_syn);
        println!("{}", pretty_actual);
        assert_eq!(pretty_actual, pretty_expect);
    }
}

use proc_macro2::TokenStream;

use crate::crate_location::CrateLocation;
use crate::syntax::SyntaxFilterStatements;
use crate::utils::ToTokenWrapperSupport;

pub fn filter(crate_location: CrateLocation, input: TokenStream) -> TokenStream {
    let ast: SyntaxFilterStatements = syn::parse2(input).unwrap();
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
            pub struct DefaultUuidFilter for Uuid impl eq + in{

            }
        };
        let expect = quote! {
            pub struct Filter {

            }
        };
        let actual = filter(CrateLocation::Outside, input);
        println!("{}", actual);
        let expect_syn: syn::File = syn::parse2(expect).unwrap();
        let actual_syn: syn::File = syn::parse2(actual).unwrap();
        let pretty_actual = unparse(&actual_syn);
        let pretty_expect = unparse(&expect_syn);
        assert_eq!(pretty_actual, pretty_expect);
    }
}

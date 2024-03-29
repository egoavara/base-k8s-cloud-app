use crate::sorter_syntax::SorterSyntax;
use proc_macro2::{Ident, TokenStream};


pub fn sorter(crate_ident: &'static str, input: TokenStream) -> TokenStream {
    let crate_ident = Ident::new(crate_ident, proc_macro2::Span::call_site());
    let ast: SorterSyntax = syn::parse2(input).unwrap();
    ast.to_tokens(&crate_ident)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use prettyplease::unparse;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::parse_quote;

    use super::*;

    #[test]
    fn tuple_style_string_sorter() {
        let input: TokenStream = parse_quote! {
            pub enum Sorter for String {
                impl asc,
                impl desc,
            }
        };
        let expect = quote! {

            pub struct Filter {
                eq: ::core::option::Option<String>,
                ne: ::core::option::Option<String>,
                r#in: ::std::vec::Vec<String>,
                between: ::core::option::Option<::table_trait::Range<String> >
            }
        };
        let actual = sorter("table_trait", input);
        println!("actual: {}", actual);
        let expect_syn: syn::File = syn::parse2(expect).unwrap();
        let actual_syn: syn::File = syn::parse2(actual).unwrap();
        let pretty_actual = unparse(&actual_syn);
        let pretty_expect = unparse(&expect_syn);
        println!("pretty_actual: {}", pretty_actual);
        assert_eq!(pretty_actual, pretty_expect);
    }
}

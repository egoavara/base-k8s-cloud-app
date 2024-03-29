use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::DeriveInput;

use crate::table_field::TableFields;

pub fn table(crate_ident: &'static str, input: TokenStream) -> TokenStream {
    let crate_ident = Ident::new(crate_ident, proc_macro2::Span::call_site());
    let ast: DeriveInput = syn::parse2(input).unwrap();

    let type_name = &ast.ident;

    let fields = TableFields::new(&crate_ident, &ast).unwrap();
    let id_fields = fields.id_field();
    let id_type = &id_fields.field.ty;

    let expanded = quote! {
        impl ::#crate_ident::Table for #type_name{
            type Id = #id_type;

            type Filter;

            type Sorting;
        }
    };

    // Convert into a token stream and return it
    expanded
}

#[cfg(test)]
mod test {
    use crate::sorter;
    use prettyplease::unparse;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::parse_quote;

    

    #[test]
    fn test_derive_table() {
        let input: TokenStream = parse_quote! {
            #[table("public", "sample")]
            pub struct Sample{
                #[table(id, sorter, filter)]
                id: Uuid,
                name: String,
                age: i32,
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
        pretty_assertions::assert_eq!(pretty_actual, pretty_expect);
    }
}

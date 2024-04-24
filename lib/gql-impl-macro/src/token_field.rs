use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::derive_table::Table;
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

pub struct TokenField<'a> {
    pub(crate) table: &'a Table,
}

impl<'a> TokenField<'a> {
    pub(crate) fn ident(&self) -> Ident {
        Ident::new(
            self.table
                .naming
                .to_field_type_name(&self.table.ident.to_string())
                .as_ref(),
            self.table.ident.span(),
        )
    }
}

impl<'a> ToTokenWrapperSupport for TokenField<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        let field_ident = self.ident();
        let data = self.table.columns();
        let field_enum_values = data.iter().map(|column| {
            let column_ident = column.ident.as_ref().unwrap();
            let enum_value = self.table.naming.to_enum_case(&column_ident.to_string());
            Ident::new(enum_value.as_ref(), column_ident.span())
        });
        let match_arms = data.iter().map(|column| {
            let column_ident = column.ident.as_ref().unwrap();
            let enum_value = self.table.naming.to_enum_case(&column_ident.to_string());
            let enum_ident = Ident::new(enum_value.as_ref(), column_ident.span());
            let db_name = self.table.naming.to_db_case(&column_ident.to_string());
            quote! {
                #field_ident::#enum_ident => ::sea_query::Alias::new(#db_name)
            }
        });
        tokens.extend(quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum #field_ident {
                #(#field_enum_values, )*
            }
            impl #crate_location::Field for #field_ident{
                fn column_ident(&self) -> ::sea_query::Alias {
                    match self{
                        #(#match_arms,)*
                    }
                }
            }
        })
    }
}

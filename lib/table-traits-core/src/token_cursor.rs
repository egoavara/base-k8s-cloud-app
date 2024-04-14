use crate::derive_table::{ColumnFilter, ColumnSorter, Table};
use crate::token_table_filter::TokenTableFilter;
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;
use darling::util::Override;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub struct TokenCursor<'a> {
    pub(crate) table: &'a Table,
}

impl<'a> TokenCursor<'a> {
    pub(crate) fn ident(&self) -> Ident {
        Ident::new(self.table.naming.to_cursor_type_name(&self.table.ident.to_string()).as_ref(), self.table.ident.span())
    }
}

impl<'a> TokenCursor<'a> {
    fn impl_cursor(&self, crate_location: CrateLocation) -> TokenStream {
        let cursor_ident = self.ident();
        let cursor_name = cursor_ident.to_string();
        quote! {
            impl #crate_location::Cursor for #cursor_ident {

                fn type_name() -> &'static str{
                    #cursor_name
                }

                fn decode(s: &str) -> ::core::result::Result<Self, #crate_location::types::CursorDecodeError>{
                    let bytes = #crate_location::prelude::data_encoding::BASE64URL_NOPAD.decode(s.as_bytes())?;
                    let data = #crate_location::prelude::postcard::from_bytes(&bytes)?;
                    ::core::result::Result::Ok(data)
                }

                fn encode(&self) -> ::std::string::String{
                    #crate_location::prelude::data_encoding::BASE64URL_NOPAD.encode(
                        &#crate_location::prelude::postcard::to_allocvec(self).unwrap()
                    )
                }

                #[inline]
                fn chunk(&self) -> ::core::option::Option<#crate_location::CursorChunk>{
                    self.__cursor_chunk.clone()
                }

                #[inline]
                fn with_chunk(self, chunk: #crate_location::CursorChunk) -> Self{
                    Self{
                        __cursor_chunk: ::core::option::Option::Some(chunk),
                        ..self
                    }
                }
            }
        }
    }
}

impl<'a> ToTokenWrapperSupport for TokenCursor<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        let cursor_ident = self.ident();
        let cursor_fields = self
            .table
            .sorter_columns()
            .into_iter()
            .map(|(field, sorter)| {
                let field_name = self.table.naming.to_field_ident(&field.ident.as_ref().unwrap());
                let field_ty = &field.ty;
                quote! {
                    pub #field_name: ::core::option::Option<#field_ty>
                }
            })
            .collect::<Vec<_>>();
        let impl_cursor_rs = self.impl_cursor(crate_location);
        tokens.extend(quote! {
            #[derive(::core::clone::Clone, ::core::fmt::Debug, ::core::default::Default, #crate_location::prelude::serde::Deserialize, #crate_location::prelude::serde::Serialize)]
            pub struct #cursor_ident{
                #(#cursor_fields,)*
                __cursor_chunk: ::core::option::Option<#crate_location::CursorChunk>,
            }
            #impl_cursor_rs
        });
    }
}

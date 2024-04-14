use darling::ast::Data;
use darling::util::Override;
use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::derive_table::{ColumnFilter, Table};
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

pub struct TokenTableFilter<'a> {
    pub(crate) table: &'a Table,
}

impl<'a> TokenTableFilter<'a> {
    pub(crate) fn ident(&self) -> Ident {
        Ident::new(
            self.table
                .naming
                .to_table_filter_type_name(&self.table.ident.to_string())
                .as_ref(),
            self.table.ident.span(),
        )
    }
}

impl<'a> TokenTableFilter<'a> {
    fn impl_table_filter(&self, crate_location: CrateLocation) -> TokenStream {
        let filter_ident = self.ident();
        let field_type = self.table.field().ident();
        let table_ident = &self.table.ident;
        let id_columns = self.table.id_columns();

        let filter_field_push = self
            .table
            .filter_columns()
            .into_iter()
            .map(|(column, _)| {
                let enum_value = self
                    .table
                    .naming
                    .to_enum_ident(&column.ident.as_ref().unwrap());
                quote! {
                    result.push(#field_type::#enum_value)
                }
            })
            .collect::<Vec<_>>();
        let conditions = self
            .table
            .filter_columns()
            .into_iter()
            .map(|(column, _)| {
                let enum_value = self.table.naming.to_enum_ident(&column.ident.as_ref().unwrap());
                let field_ident = self.table.naming.to_field_ident(&column.ident.as_ref().unwrap());
                quote! {
                    if let ::core::option::Option::Some(value) = &self.#field_ident{
                        result = result.add(#crate_location::Filter::build_all_condition(value, context.column_ref(#field_type::#enum_value.column_ident())));
                    }
                }
            })
            .collect::<Vec<_>>();

        let id_field_mapper = if id_columns.len() == 1 {
            let first = id_columns.first().unwrap();
            let field_ident = self
                .table
                .naming
                .to_field_ident(first.ident.as_ref().unwrap());
            vec![quote! {
                #field_ident: Some(#crate_location::FilterImpl::filter_by_id(&id))
            }]
        } else {
            self.table
                .id_columns()
                .into_iter()
                .enumerate()
                .map(|(i, column)| {
                    let field_ident = self
                        .table
                        .naming
                        .to_field_ident(&column.ident.as_ref().unwrap());
                    quote! {
                        #field_ident: Some(#crate_location::FilterImpl::filter_by_id(&id.#i))
                    }
                })
                .collect_vec()
        };

        quote! {
            impl #crate_location::TableFilter for #filter_ident{

                type Table = #table_ident;

                fn filter_fields() -> Vec<<Self::Table as #crate_location::FieldMetadata>::Field>{
                    let mut result = ::std::vec::Vec::new();
                    #(#filter_field_push;)*
                    result
                }

                fn by_id(id: <Self::Table as #crate_location::Table>::Id) -> Self{
                    Self{
                        #(#id_field_mapper,)*
                        ..Default::default()
                    }
                }
                fn to_condition<'a, 'b>(&self, context: &'a mut #crate_location::types::Context<'b, Self::Table>) -> ::sea_query::Condition{
                    let mut result = ::sea_query::Condition::all();
                    #(#conditions)*
                    result
                }
            }
        }
    }
}
impl<'a> ToTokenWrapperSupport for TokenTableFilter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        let filter_ident = self.ident();
        let filter_fields = self.table
            .filter_columns()
            .into_iter()
            .map(|(field, filter)| {
                let field_name = self.table.naming.to_field_ident(&field.ident.as_ref().unwrap());
                let field_ty = &field.ty;
                match filter {
                    Override::Inherit | Override::Explicit(ColumnFilter::Default) => {
                        quote! {
                            pub #field_name: ::core::option::Option<<#field_ty as #crate_location::FilterImpl>::DefaultFilter>
                        }
                    }
                    Override::Explicit(ColumnFilter::With(_)) => {
                        let locality_filter_type = self.table.naming.to_filter_type_ident(&self.table.ident, &field.ident.as_ref().unwrap());
                        quote! {
                            pub #field_name: ::core::option::Option<#locality_filter_type>
                        }
                    }
                    Override::Explicit(ColumnFilter::By(passed)) => {
                        quote! {
                            pub #field_name: ::core::option::Option<#passed>
                        }
                    }
                }

            })
            .collect::<Vec<_>>();
        let impl_table_filter_rs = self.impl_table_filter(crate_location);
        tokens.extend(quote! {
            #[derive(Clone, Debug, Default, ::async_graphql::InputObject, ::serde::Deserialize, ::serde::Serialize)]
            pub struct #filter_ident{
                #(#filter_fields,)*
            }

            #impl_table_filter_rs
        });
    }
}

use darling::ast::Data;
use darling::util::Override;
use proc_macro2::TokenStream;
use quote::quote;

use crate::derive_table::{Column, ColumnFilter, ColumnSorter, Table};
use crate::token_cursor::TokenCursor;
use crate::token_field::TokenField;
use crate::token_filter::TokenFilter;
use crate::token_sorter::TokenSorter;
use crate::token_table_filter::TokenTableFilter;
use crate::token_table_sorter::TokenTableSorter;
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

impl Table {
    pub(crate) fn id_columns(&self) -> Vec<&Column> {
        match &self.data {
            Data::Struct(data) => data.iter().filter(|column| column.id).collect(),
            _ => unreachable!(),
        }
    }

    pub(crate) fn columns(&self) -> Vec<&Column> {
        match &self.data {
            Data::Struct(data) => data.iter().collect(),
            _ => unreachable!(),
        }
    }
    pub(crate) fn filter_columns(&self) -> Vec<(&Column, Override<&ColumnFilter>)> {
        match &self.data {
            Data::Struct(data) => data
                .iter()
                // id columns always have filter (if not explicitly set, it will be inherit-filter)
                .filter_map(|column| match &column.filter {
                    None => None,
                    Some(data) => Some((column, data.as_ref())),
                })
                .collect(),
            _ => unreachable!(),
        }
    }
    pub(crate) fn sorter_columns(&self) -> Vec<(&Column, Override<&ColumnSorter>)> {
        match &self.data {
            Data::Struct(data) => data
                .iter()
                // id columns always have filter (if not explicitly set, it will be inherit-filter)
                .filter_map(|column| match &column.sorter {
                    None => None,
                    Some(sorter) => Some((column, sorter.as_ref())),
                })
                .collect(),
            _ => unreachable!(),
        }
    }
    pub(crate) fn id_type(&self) -> TokenStream {
        let id_columns = self.id_columns();
        if id_columns.len() == 1 {
            let first = &id_columns.first().unwrap().ty;
            quote! {#first}
        } else {
            let id_columns = id_columns.iter().map(|column| &column.ty);
            quote! {
                (#(#id_columns),*)
            }
        }
    }
    pub(crate) fn field(&self) -> TokenField {
        TokenField { table: self }
    }
    pub(crate) fn table_filter(&self) -> TokenTableFilter {
        TokenTableFilter { table: self }
    }
    pub(crate) fn table_sorter(&self) -> TokenTableSorter {
        TokenTableSorter { table: self }
    }
    pub(crate) fn cursor(&self) -> TokenCursor {
        TokenCursor { table: self }
    }
    pub(crate) fn filters(&self) -> Vec<TokenFilter> {
        match &self.data {
            Data::Enum(_) => unreachable!(),
            Data::Struct(data) => data
                .iter()
                .filter_map(|column| match &column.filter {
                    None | Some(Override::Inherit) | Some(Override::Explicit(ColumnFilter::Default)) | Some(Override::Explicit(ColumnFilter::By(_))) => None,
                    Some(Override::Explicit(ColumnFilter::With(with))) => Some((column, with)),
                })
                .map(|(column, with)| TokenFilter { table: self, column, with })
                .collect(),
        }
    }
    pub(crate) fn sorters(&self) -> Vec<TokenSorter> {
        match &self.data {
            Data::Enum(_) => unreachable!(),
            Data::Struct(data) => data
                .iter()
                .filter_map(|column| match &column.sorter {
                    None | Some(Override::Inherit) | Some(Override::Explicit(ColumnSorter::Default)) | Some(Override::Explicit(ColumnSorter::By(_))) => None,
                    Some(Override::Explicit(ColumnSorter::With(with))) => Some((column, with)),
                })
                .map(|(column, with)| TokenSorter { table: self, column, with })
                .collect(),
        }
    }
}

// private implementation details
impl Table {
    fn impl_field_metadata(&self, crate_location: CrateLocation) -> TokenStream {
        let table_ident = &self.ident;
        let field_ident = self.field().ident();
        let value_pushes = self.columns().into_iter().map(|column| {
            let enum_value = self.naming.to_enum_ident(&column.ident.as_ref().unwrap());
            quote! {
                result.push(#field_ident::#enum_value);
            }
        });

        quote! {
            impl #crate_location::FieldMetadata for #table_ident{

                type Field = #field_ident;

                fn fields() -> ::std::vec::Vec<Self::Field> {
                    let mut result = ::std::vec::Vec::new();
                    #(#value_pushes)*
                    result
                }
            }
        }
    }
    fn impl_field_getter(&self, crate_location: CrateLocation) -> TokenStream {
        let table_ident = &self.ident;
        let field_ident = self.field().ident();
        let columns_and_namings = self
            .columns()
            .into_iter()
            .map(|column| {
                let ident = column.ident.as_ref().unwrap();
                let column_ident = self.naming.to_field_ident(ident);
                let enum_value = self.naming.to_enum_ident(ident);
                (column, column_ident, enum_value)
            })
            .collect::<Vec<_>>();
        let fvalue_graphql = columns_and_namings.iter().map(|(column, column_ident, enum_value)| {
            quote! {
                #field_ident::#enum_value => #crate_location::private::InternalConverter::to_graph(&self.#column_ident)
            }
        });
        let fvalue_json = columns_and_namings.iter().map(|(column, column_ident, enum_value)| {
            quote! {
                #field_ident::#enum_value => #crate_location::private::InternalConverter::to_json(&self.#column_ident)
            }
        });
        quote! {
           impl #crate_location::FieldGetter for #table_ident {

               fn get_field_value_graphql(&self, field: Self::Field) -> async_graphql_value::ConstValue {
                   match field{
                       #(#fvalue_graphql,)*
                   }
               }

               fn get_field_value_json(&self, field: Self::Field) -> serde_json::Value{
                   match field{
                       #(#fvalue_json,)*
                   }
               }
           }
        }
    }

    fn impl_table(&self, crate_location: CrateLocation) -> TokenStream {
        let table_ident = &self.ident;
        let table_lit = table_ident.to_string();
        let field_type = self.field().ident();
        let filter_ident = self.table_filter().ident();
        let cursor_type = self.cursor().ident();
        let sorter_type = self.table_sorter().ident();
        let id_type = self.id_type();

        let schema_lit = self.naming.to_schema_name(&self.schema, "public");
        let table_lit = self.naming.to_schema_name(&self.table, &table_lit);
        let cursor_fields = self.sorter_columns().into_iter().map(|(column, _)| {
            let field_name = self.naming.to_field_ident(&column.ident.as_ref().unwrap());
            quote! {
                #field_name: ::core::option::Option::Some(self.#field_name.clone())
            }
        });
        let id_fields_rs = self.id_columns().into_iter().map(|column| {
            let enum_value = self.naming.to_enum_ident(&column.ident.as_ref().unwrap());
            quote! {
                #field_type::#enum_value
            }
        });

        quote! {
            impl #crate_location::Table for #table_ident{
                type Id = #id_type;
                type Filter = #filter_ident;
                type Cursor = #cursor_type;
                type Sorter = #sorter_type;

                fn table() -> TableRef{
                    ::sea_query::TableRef::SchemaTable(
                        ::sea_query::DynIden::new(Alias::new(#schema_lit)),
                        ::sea_query::DynIden::new(Alias::new(#table_lit)),
                    )
                }
                fn id_fields() -> Vec<Self::Field>{
                    vec![#(#id_fields_rs,)*]
                }

                fn to_cursor(&self) -> Self::Cursor{
                    Self::Cursor{
                        #(#cursor_fields,)*
                        __cursor_chunk: None,
                    }
                }
            }
        }
    }
}

impl ToTokenWrapperSupport for Table {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        let field = self.field();
        let filters = self.filters();
        let sorters = self.sorters();
        let table_filter = self.table_filter();
        let cursor = self.cursor();
        let table_sorter = self.table_sorter();

        let field_rs = field.to_token_stream(crate_location);
        let filters_rs = filters.iter().fold(TokenStream::new(), |mut acc, fold| {
            fold.to_tokens(&mut acc, crate_location);
            acc
        });
        let table_filter_rs = table_filter.to_token_stream(crate_location);
        let cursor_rs = cursor.to_token_stream(crate_location);
        let sorters_rs = sorters.iter().fold(TokenStream::new(), |mut acc, fold| {
            fold.to_tokens(&mut acc, crate_location);
            acc
        });
        let table_sorter_rs = table_sorter.to_token_stream(crate_location);

        let impl_field_type_metadata_rs = self.impl_field_metadata(crate_location);
        let impl_field_type_getter_rs = self.impl_field_getter(crate_location);
        let impl_field_table_rs = self.impl_table(crate_location);
        tokens.extend(quote! {
            #field_rs

            #filters_rs

            #table_filter_rs

            #cursor_rs

            #sorters_rs

            #table_sorter_rs

            #impl_field_type_metadata_rs
            #impl_field_type_getter_rs
            #impl_field_table_rs
        })
    }
}

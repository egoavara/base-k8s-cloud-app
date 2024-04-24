use darling::util::Override;
use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::derive_table::{ColumnSorter, Table};
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

pub struct TokenTableSorter<'a> {
    pub(crate) table: &'a Table,
}

impl<'a> TokenTableSorter<'a> {
    pub(crate) fn elem_ident(&self) -> Ident {
        Ident::new(self.table.naming.to_table_sorter_elem_type_name(&self.table.ident.to_string()).as_ref(), self.table.ident.span())
    }
    pub(crate) fn ident(&self) -> Ident {
        Ident::new(self.table.naming.to_table_sorter_type_name(&self.table.ident.to_string()).as_ref(), self.table.ident.span())
    }
}

impl<'a> TokenTableSorter<'a> {
    fn variant_table_sorter(&self, crate_location: CrateLocation) -> Vec<TokenStream> {
        self.table
            .sorter_columns()
            .into_iter()
            .map(|(column, column_sorter)| {
                let field_enum_value = self.table.naming.to_enum_ident(&column.ident.as_ref().unwrap());
                let field_ty = &column.ty;

                match column_sorter {
                    Override::Inherit | Override::Explicit(ColumnSorter::Default) => {
                        quote! {
                            #field_enum_value(<#field_ty as #crate_location::SorterImpl>::DefaultSorter)
                        }
                    }
                    Override::Explicit(ColumnSorter::With(_)) => {
                        let field_ident = column.ident.as_ref().unwrap();
                        let field_ty_name = self.table.naming.to_locality_sorter_type_name(self.table.ident.to_string().as_str(), field_ident.to_string().as_str());
                        let field_ty_ident = Ident::new(field_ty_name.as_str(), field_ident.span());
                        quote! {
                            #field_enum_value(#field_ty_ident)
                        }
                    }
                    Override::Explicit(ColumnSorter::By(by)) => {
                        quote! {
                            #field_enum_value(#by)
                        }
                    }
                }
            })
            .collect()
    }
    fn impl_table_sorter(&self, crate_location: CrateLocation) -> TokenStream {
        let field_type = self.table.field().ident();
        let sorter_ident = self.ident();
        let sorter_elem_ident = self.elem_ident();
        let table_ident = &self.table.ident;
        let sorter_fields_push_rs = self.table.sorter_columns().into_iter().map(|(column, _)| {
            let enum_value = self.table.naming.to_enum_ident(&column.ident.as_ref().unwrap());
            quote! {
                result.push(#field_type::#enum_value);
            }
        });

        let impl_after = self.impl_build_condition(crate_location, "build_after", "build_equal");
        let impl_before = self.impl_build_condition(crate_location, "build_before", "build_equal");
        let impl_has_after = self.impl_build_has_condition(crate_location, "build_after", "build_equal");
        let impl_has_before = self.impl_build_has_condition(crate_location, "build_before", "build_equal");

        let matcharm_sorter_order_rs = self.table.sorter_columns().into_iter().map(|(column, _)| {
            let column_ident = column.ident.as_ref().unwrap();
            let enum_value = self.table.naming.to_enum_ident(column_ident);

            quote! {
                #sorter_elem_ident::#enum_value(sorter) => {
                    result.push(#crate_location::Sorter::build_order(sorter, context.column_ref(#field_type::#enum_value.column_ident())));
                }
            }
        });

        quote! {
            impl #crate_location::TableSorter for #sorter_ident{

                type Table = #table_ident;

                fn sorter_fields() -> ::std::vec::Vec<<Self::Table as #crate_location::FieldMetadata>::Field>{
                    let mut result = ::std::vec::Vec::new();
                    #(#sorter_fields_push_rs)*
                    result
                }

                fn to_after_condition<'a, 'b>(&self, cursor: <Self::Table as #crate_location::Table>::Cursor, context: &'a mut #crate_location::types::Context<'b, Self::Table>) -> ::sea_query::Condition{
                    #impl_after
                }

                fn to_before_condition<'a, 'b>(&self, cursor: <Self::Table as #crate_location::Table>::Cursor, context: &'a mut #crate_location::types::Context<'b, Self::Table>) -> ::sea_query::Condition{
                    #impl_before
                }

                fn to_has_after_condition<'a, 'b>(&self, cursor: <Self::Table as #crate_location::Table>::Cursor, context: &'a mut #crate_location::types::Context<'b, Self::Table>) -> ::sea_query::Condition{
                    #impl_has_after
                }

                fn to_has_before_condition<'a, 'b>(&self, cursor: <Self::Table as #crate_location::Table>::Cursor, context: &'a mut #crate_location::types::Context<'b, Self::Table>) -> ::sea_query::Condition{
                    #impl_has_before
                }

                fn to_order<'a, 'b>(&self, context: &'a mut #crate_location::types::Context<'b, Self::Table>) -> ::std::vec::Vec<(::sea_query::SimpleExpr, ::sea_query::Order, ::core::option::Option<::sea_query::NullOrdering>)>{
                    let mut result = ::std::vec::Vec::new();
                    for field in &self.0{
                        match field{
                            #(#matcharm_sorter_order_rs,)*
                            _ => {},
                        }
                    }
                    result
                }
            }
        }
    }

    fn impl_build_condition(&self, crate_location: CrateLocation, condition_method: &'static str, equal_method: &'static str) -> TokenStream {
        let field_type = self.table.field().ident();
        let sorter_elem_ident = self.elem_ident();
        let condition_method = Ident::new(condition_method, Span::call_site());
        let equal_method = Ident::new(equal_method, Span::call_site());
        let matcharm_i_sorter_cond_rs = self.table.sorter_columns().into_iter().map(|(column, _)| {
            let column_ident = column.ident.as_ref().unwrap();
            let enum_value = self.table.naming.to_enum_ident(column_ident);
            let field_ident = self.table.naming.to_field_ident(column_ident);

            quote! {
                #sorter_elem_ident::#enum_value(isorter) => {
                    if let ::core::option::Option::Some(value) = &cursor.#field_ident{
                        temp = temp.add(#crate_location::Sorter::#condition_method(isorter, value.clone(), context.column_ref(#field_type::#enum_value.column_ident())));
                    }else{
                        break;
                    }
                }
            }
        });
        let matcharm_j_sorter_cond_rs = self.table.sorter_columns().into_iter().map(|(column, _)| {
            let column_ident = column.ident.as_ref().unwrap();
            let enum_value = self.table.naming.to_enum_ident(column_ident);
            let field_ident = self.table.naming.to_field_ident(column_ident);

            quote! {
                #sorter_elem_ident::#enum_value(jsorter) => {
                    if let ::core::option::Option::Some(value) = &cursor.#field_ident{
                        temp = temp.add(#crate_location::Sorter::#equal_method(jsorter, value.clone(), context.column_ref(#field_type::#enum_value.column_ident())));
                    }else{
                        break;
                    }
                }
            }
        });
        quote! {
            let mut result = ::sea_query::Condition::any();
            for i in 0..(self.0.len()){
                let mut temp = ::sea_query::Condition::all();
                let ifield = &self.0[i];
                match ifield {
                    #(#matcharm_i_sorter_cond_rs,)*
                    _ => {},
                }
                for j in 0..i{
                    let jfield = &self.0[j];
                    match jfield {
                        #(#matcharm_j_sorter_cond_rs,)*
                        _ => {},
                    }
                }
                result = result.add(temp);
            }
            result
        }
    }

    fn impl_build_has_condition(&self, crate_location: CrateLocation, condition_method: &'static str, equal_method: &'static str) -> TokenStream {
        let field_type = self.table.field().ident();
        let sorter_elem_ident = self.elem_ident();
        let condition_method = Ident::new(condition_method, Span::call_site());
        let equal_method = Ident::new(equal_method, Span::call_site());
        let matcharm_i_sorter_cond_rs = self.table.sorter_columns().into_iter().map(|(column, _)| {
            let column_ident = column.ident.as_ref().unwrap();
            let enum_value = self.table.naming.to_enum_ident(column_ident);
            let field_ident = self.table.naming.to_field_ident(column_ident);

            quote! {
                #sorter_elem_ident::#enum_value(isorter) => {
                    if let ::core::option::Option::Some(value) = &cursor.#field_ident{
                        temp = temp.add(#crate_location::Sorter::#condition_method(isorter, value.clone(), context.column_ref(#field_type::#enum_value.column_ident())));
                    }else{
                        break;
                    }
                }
            }
        });
        let matcharm_j_sorter_cond_rs = self
            .table
            .sorter_columns()
            .into_iter()
            .map(|(column, _)| {
                let column_ident = column.ident.as_ref().unwrap();
                let enum_value = self.table.naming.to_enum_ident(column_ident);
                let field_ident = self.table.naming.to_field_ident(column_ident);

                quote! {
                    #sorter_elem_ident::#enum_value(jsorter) => {
                        if let ::core::option::Option::Some(value) = &cursor.#field_ident{
                            temp = temp.add(#crate_location::Sorter::#equal_method(jsorter, value.clone(), context.column_ref(#field_type::#enum_value.column_ident())));
                        }else{
                            break;
                        }
                    }
                }
            })
            .collect_vec();
        quote! {
            let mut result = ::sea_query::Condition::any();
            for i in 0..(self.0.len()){
                let mut temp = ::sea_query::Condition::all();
                let ifield = &self.0[i];
                match ifield {
                    #(#matcharm_i_sorter_cond_rs,)*
                    _ => {},
                }
                for j in 0..i{
                    let jfield = &self.0[j];
                    match jfield {
                        #(#matcharm_j_sorter_cond_rs,)*
                        _ => {},
                    }
                }
                result = result.add(temp);
            }
            // all equal case
            let mut temp = ::sea_query::Condition::all();
            for j in 0..(self.0.len()){
                let jfield = &self.0[j];
                match jfield {
                    #(#matcharm_j_sorter_cond_rs,)*
                    _ => {},
                }
            }
            result = result.add(temp);
            result
        }
    }

    fn impl_input_type(&self) -> TokenStream {
        let sorter_ident = self.ident();
        let sorter_elem_ident = self.elem_ident();
        quote! {
            impl ::async_graphql::InputType for #sorter_ident {
                type RawValueType = ::std::vec::Vec<#sorter_elem_ident>;

                fn type_name() -> ::std::borrow::Cow<'static, str> {
                    ::std::borrow::Cow::Owned(format!("[{}]", #sorter_elem_ident::qualified_type_name()))
                }

                fn qualified_type_name() -> ::std::string::String {
                    format!("[{}]!", #sorter_elem_ident::qualified_type_name())
                }

                fn create_type_info(registry: &mut ::async_graphql::registry::Registry) -> ::std::string::String {
                    #sorter_elem_ident::create_type_info(registry);
                    Self::qualified_type_name()
                }

                fn parse(value: ::core::option::Option<::async_graphql::Value>) -> ::async_graphql::InputValueResult<Self> {
                    match value.unwrap_or_default() {
                        ::async_graphql::Value::List(values) => values
                            .into_iter()
                            .map(|value| ::async_graphql::InputType::parse(::core::option::Option::Some(value)))
                            .collect::<async_graphql::Result<_, _>>()
                            .map(|x| #sorter_ident(x))
                            .map_err(::async_graphql::InputValueError::propagate),
                        value => Ok(::async_graphql::InputType::parse(::core::option::Option::Some(value))
                            .map(|x| #sorter_ident(vec![x]))
                            .map_err(::async_graphql::InputValueError::propagate)?),
                    }
                }

                fn to_value(&self) -> Value {
                    ::async_graphql::Value::List(self.0.iter().map(::async_graphql::InputType::to_value).collect())
                }

                fn as_raw_value(&self) -> ::core::option::Option<&Self::RawValueType> {
                    ::core::option::Option::Some(&self.0)
                }
            }
        }
    }
}
impl<'a> ToTokenWrapperSupport for TokenTableSorter<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream, crate_location: CrateLocation) {
        let sorter_ident = self.ident();
        let sorter_elem_ident = self.elem_ident();
        let variant_table_sorter_rs = self.variant_table_sorter(crate_location);

        let impl_input_type_rs = self.impl_input_type();
        let impl_table_sorter_rs = self.impl_table_sorter(crate_location);

        tokens.extend(quote! {
            #[derive(
                ::core::clone::Clone,
                ::core::fmt::Debug,
                ::async_graphql::OneofObject,
                ::serde::Deserialize,
                ::serde::Serialize
            )]
            pub enum #sorter_elem_ident{
                #(#variant_table_sorter_rs,)*
            }

            #[derive(
                ::core::clone::Clone,
                ::core::fmt::Debug,
                ::serde::Deserialize,
                ::serde::Serialize
            )]
            pub struct #sorter_ident(pub ::std::vec::Vec<#sorter_elem_ident>);

            #impl_input_type_rs
            #impl_table_sorter_rs
        });
    }
}

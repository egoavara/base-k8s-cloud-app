use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;


#[derive(Debug)]
pub enum SorterKind {
    Asc(Ident),
    Desc(Ident),
    None(Ident),
}

impl Parse for SorterKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        match ident.to_string().as_str() {
            "asc" => Ok(SorterKind::Asc(ident)),
            "desc" => Ok(SorterKind::Desc(ident)),
            "none" => Ok(SorterKind::Desc(ident)),
            _ => Err(syn::Error::new(
                ident.span(),
                format!("Not allowed filter implements: {}", ident),
            )),
        }
    }
}

impl SorterKind {
    pub fn implemented_filters_expr(&self, crate_ident: &Ident) -> TokenStream {
        let en = self.enum_ident();
        quote! {
            result.push(::#crate_ident::SorterKind::#en);
        }
    }
    pub fn activated_sorter_match_arm(&self, crate_ident: &Ident) -> TokenStream {
        let en = self.enum_ident();
        quote! {
            Self::#en => ::#crate_ident::SorterKind::#en,
        }
    }
    pub fn sorter_value_match_arm(&self, crate_ident: &Ident) -> TokenStream {
        let en = self.enum_ident();
        match self {
            SorterKind::Asc(_) | SorterKind::Desc(_) => quote! {
                ::#crate_ident::SorterKind::#en => ::#crate_ident::SorterValue::#en(value),
            },
            SorterKind::None(_) => quote! {
                ::#crate_ident::SorterKind::#en => ::#crate_ident::SorterValue::#en,
            },
        }
    }

    pub fn to_variant(&self) -> TokenStream {
        let en = self.enum_ident();
        match self {
            SorterKind::Asc(_) | SorterKind::Desc(_) | SorterKind::None(_) => {
                quote! {#en}
            }
        }
    }
    pub fn field_ident(&self) -> Ident {
        match self {
            SorterKind::Asc(ident) => Ident::new_raw("asc", ident.span()),
            SorterKind::Desc(ident) => Ident::new_raw("desc", ident.span()),
            SorterKind::None(ident) => Ident::new_raw("none", ident.span()),
        }
    }
    pub fn enum_ident(&self) -> Ident {
        match self {
            SorterKind::Asc(token) => Ident::new("Asc", token.span()),
            SorterKind::Desc(ident) => Ident::new("Desc", ident.span()),
            SorterKind::None(ident) => Ident::new("None", ident.span()),
        }
    }
}

pub enum Sorter {
    Asc,
    Desc,
    None,
}

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Token, Type};

#[derive(Debug)]
pub enum FilterKind {
    // equality
    Eq(Ident),
    Ne(Ident),
    // comparison
    Gt(Ident),
    Lt(Ident),
    Gte(Ident),
    Lte(Ident),
    // set
    In(Token![in]),
    Nin(Ident),
    // like
    Like(Ident),
    Nlike(Ident),
    // null
    Null(Ident),
    Nonnull(Ident),
    // between
    Between(Ident),
    NBetween(Ident),
    // string matches
    Prefix(Ident),
    NPrefix(Ident),
    Suffix(Ident),
    NSuffix(Ident),
    Contains(Ident),
    NContains(Ident),
    // regex
    Regex(Ident),
}

impl Parse for FilterKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![in]) {
            let in_token = input.parse::<Token![in]>()?;
            return Ok(FilterKind::In(in_token));
        }
        if lookahead.peek(syn::Ident) {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "eq" => Ok(FilterKind::Eq(ident)),
                "ne" => Ok(FilterKind::Ne(ident)),
                "gt" => Ok(FilterKind::Gt(ident)),
                "lt" => Ok(FilterKind::Lt(ident)),
                "gte" => Ok(FilterKind::Gte(ident)),
                "lte" => Ok(FilterKind::Lte(ident)),
                "nin" => Ok(FilterKind::Nin(ident)),
                "like" => Ok(FilterKind::Like(ident)),
                "nlike" => Ok(FilterKind::Nlike(ident)),
                "null" => Ok(FilterKind::Null(ident)),
                "nonnull" => Ok(FilterKind::Nonnull(ident)),
                "between" => Ok(FilterKind::Between(ident)),
                "nbetween" => Ok(FilterKind::NBetween(ident)),
                "prefix" => Ok(FilterKind::Prefix(ident)),
                "nprefix" => Ok(FilterKind::NPrefix(ident)),
                "suffix" => Ok(FilterKind::Suffix(ident)),
                "nsuffix" => Ok(FilterKind::NSuffix(ident)),
                "contains" => Ok(FilterKind::Contains(ident)),
                "ncontains" => Ok(FilterKind::NContains(ident)),
                "regex" => Ok(FilterKind::Regex(ident)),
                _ => Err(syn::Error::new(
                    ident.span(),
                    format!("Not allowed filter implements: {}", ident),
                )),
            }
        } else {
            Err(lookahead.error())
        }
    }
}

impl FilterKind {
    pub fn implemented_filters_expr(&self, crate_ident: &Ident) -> TokenStream {
        let en = self.enum_ident();
        quote! {
            result.push(::#crate_ident::FilterKind::#en);
        }
    }
    pub fn activated_filters_expr(&self, crate_ident: &Ident) -> TokenStream {
        let en = self.enum_ident();
        let fd = self.field_ident();
        match self {
            // 1 parameter filters
            FilterKind::Eq(_)
            | FilterKind::Ne(_)
            | FilterKind::Gt(_)
            | FilterKind::Lt(_)
            | FilterKind::Gte(_)
            | FilterKind::Lte(_)
            | FilterKind::Like(_)
            | FilterKind::Nlike(_)
            | FilterKind::Null(_)
            | FilterKind::Nonnull(_)
            | FilterKind::Prefix(_)
            | FilterKind::NPrefix(_)
            | FilterKind::Suffix(_)
            | FilterKind::NSuffix(_)
            | FilterKind::Contains(_)
            | FilterKind::NContains(_)
            | FilterKind::Regex(_) => {
                quote! {
                    if self.#fd.is_some(){
                        result.push(::#crate_ident::FilterKind::#en);
                    }
                }
            }
            // 1 vector parameter filters
            FilterKind::In(_) | FilterKind::Nin(_) => {
                quote! {
                    if !self.#fd.is_empty(){
                        result.push(::#crate_ident::FilterKind::#en);
                    }
                }
            }
            // 2 parameter filters
            FilterKind::Between(_) | FilterKind::NBetween(_) => quote! {
                if self.#fd.is_some(){
                    result.push(::#crate_ident::FilterKind::#en);
                }
            },
        }
    }
    pub fn filter_value_match_arm(&self, crate_ident: &Ident) -> TokenStream {
        let en = self.enum_ident();
        let fd = self.field_ident();
        match self {
            // 1 parameter filters
            FilterKind::Eq(_)
            | FilterKind::Ne(_)
            | FilterKind::Gt(_)
            | FilterKind::Lt(_)
            | FilterKind::Gte(_)
            | FilterKind::Lte(_)
            | FilterKind::Like(_)
            | FilterKind::Nlike(_)
            | FilterKind::Null(_)
            | FilterKind::Nonnull(_)
            | FilterKind::Prefix(_)
            | FilterKind::NPrefix(_)
            | FilterKind::Suffix(_)
            | FilterKind::NSuffix(_)
            | FilterKind::Contains(_)
            | FilterKind::NContains(_)
            | FilterKind::Regex(_) => {
                quote! {
                    ::#crate_ident::FilterKind::#en => match & self.#fd {
                        ::core::option::Option::Some(value) => ::#crate_ident::FilterValue::#en(value.clone()),
                        ::core::option::Option::None => ::#crate_ident::FilterValue::None,
                    }
                }
            }
            // 1 vector parameter filters
            FilterKind::In(_) | FilterKind::Nin(_) => {
                quote! {
                    ::#crate_ident::FilterKind::#en => if self.#fd.len() > 0 { ::#crate_ident::FilterValue::#en(self.#fd.clone()) } else { ::#crate_ident::FilterValue::None }
                }
            }
            // 2 parameter filters
            FilterKind::Between(_) | FilterKind::NBetween(_) => quote! {
                ::#crate_ident::FilterKind::#en => match &self.#fd {
                    ::core::option::Option::Some(value) => ::#crate_ident::FilterValue::#en(value.min.clone(), value.max.clone()),
                    ::core::option::Option::None => ::#crate_ident::FilterValue::None,
                }
            },
        }
    }
    pub fn field_ty(&self, crate_ident: &Ident, target_ty: &Type) -> Type {
        match self {
            FilterKind::In(_) | FilterKind::Nin(_) => {
                syn::parse2(quote!(::std::vec::Vec<#target_ty>)).unwrap()
            }
            FilterKind::Eq(_)
            | FilterKind::Ne(_)
            | FilterKind::Gt(_)
            | FilterKind::Lt(_)
            | FilterKind::Gte(_)
            | FilterKind::Lte(_)
            | FilterKind::Like(_)
            | FilterKind::Nlike(_)
            | FilterKind::Null(_)
            | FilterKind::Nonnull(_)
            | FilterKind::Prefix(_)
            | FilterKind::NPrefix(_)
            | FilterKind::Suffix(_)
            | FilterKind::NSuffix(_)
            | FilterKind::Contains(_)
            | FilterKind::NContains(_)
            | FilterKind::Regex(_) => {
                syn::parse2(quote!(::core::option::Option<#target_ty>)).unwrap()
            }
            FilterKind::Between(_) | FilterKind::NBetween(_) => {
                syn::parse2(quote!(::core::option::Option<::#crate_ident::Range<#target_ty>>))
                    .unwrap()
            }
        }
    }
    pub fn field_ident(&self) -> Ident {
        match self {
            FilterKind::In(token) => Ident::new_raw("in", token.span),
            FilterKind::Eq(ident)
            | FilterKind::Ne(ident)
            | FilterKind::Gt(ident)
            | FilterKind::Lt(ident)
            | FilterKind::Gte(ident)
            | FilterKind::Lte(ident)
            | FilterKind::Nin(ident)
            | FilterKind::Like(ident)
            | FilterKind::Nlike(ident)
            | FilterKind::Null(ident)
            | FilterKind::Nonnull(ident)
            | FilterKind::Between(ident)
            | FilterKind::NBetween(ident)
            | FilterKind::Prefix(ident)
            | FilterKind::NPrefix(ident)
            | FilterKind::Suffix(ident)
            | FilterKind::NSuffix(ident)
            | FilterKind::Contains(ident)
            | FilterKind::NContains(ident)
            | FilterKind::Regex(ident) => ident.clone(),
        }
    }
    pub fn enum_ident(&self) -> Ident {
        match self {
            FilterKind::In(token) => Ident::new("In", token.span()),
            FilterKind::Eq(ident) => Ident::new("Eq", ident.span()),
            FilterKind::Ne(ident) => Ident::new("Ne", ident.span()),
            FilterKind::Gt(ident) => Ident::new("Gt", ident.span()),
            FilterKind::Lt(ident) => Ident::new("Lt", ident.span()),
            FilterKind::Gte(ident) => Ident::new("Gte", ident.span()),
            FilterKind::Lte(ident) => Ident::new("Lte", ident.span()),
            FilterKind::Nin(ident) => Ident::new("Nin", ident.span()),
            FilterKind::Like(ident) => Ident::new("Like", ident.span()),
            FilterKind::Nlike(ident) => Ident::new("Nlike", ident.span()),
            FilterKind::Null(ident) => Ident::new("Null", ident.span()),
            FilterKind::Nonnull(ident) => Ident::new("Nonnull", ident.span()),
            FilterKind::Between(ident) => Ident::new("Between", ident.span()),
            FilterKind::NBetween(ident) => Ident::new("NBetween", ident.span()),
            FilterKind::Prefix(ident) => Ident::new("Prefix", ident.span()),
            FilterKind::NPrefix(ident) => Ident::new("NPrefix", ident.span()),
            FilterKind::Suffix(ident) => Ident::new("Suffix", ident.span()),
            FilterKind::NSuffix(ident) => Ident::new("NSuffix", ident.span()),
            FilterKind::Contains(ident) => Ident::new("Contains", ident.span()),
            FilterKind::NContains(ident) => Ident::new("NContains", ident.span()),
            FilterKind::Regex(ident) => Ident::new("Regex", ident.span()),
        }
    }
}

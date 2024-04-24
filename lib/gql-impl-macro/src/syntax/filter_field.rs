use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, token, Attribute, FieldMutability, Token, Type, Visibility};

use crate::syntax::filter_kind::SyntaxFilterKind;
use crate::utils::ToTokenWrapperSupport;
use crate::CrateLocation;

pub struct SyntaxFilterFields {
    pub brace_token: token::Brace,
    pub named: Punctuated<SyntaxFilterField, Token![,]>,
}

pub struct SyntaxFilterField {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: FieldMutability,
    pub ident: Ident,
    pub colon_token: Token![:],
    pub ty: SyntaxFieldType,
}

pub enum SyntaxFieldType {
    Impl(SyntaxFilterKind),
    Type(Type),
}

impl Parse for SyntaxFilterFields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace_token = braced!(content in input);
        let named = content.parse_terminated(SyntaxFilterField::parse, Token![,])?;
        Ok(Self { brace_token, named })
    }
}

impl Parse for SyntaxFilterField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let ident = input.parse()?;
        let colon_token = input.parse()?;
        let ty = input.parse()?;
        Ok(Self {
            attrs,
            vis,
            mutability: FieldMutability::None,
            ident,
            colon_token,
            ty,
        })
    }
}
impl Parse for SyntaxFieldType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![impl]) {
            input.parse::<Token![impl]>()?;
            let kind = input.parse::<SyntaxFilterKind>()?;
            Ok(SyntaxFieldType::Impl(kind))
        } else {
            Ok(SyntaxFieldType::Type(input.parse()?))
        }
    }
}

impl SyntaxFilterField {
    pub fn to_token_stream(&self, target_ty: &Type, crate_location: CrateLocation) -> TokenStream {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let ident = &self.ident;
        let colon_token = &self.colon_token;
        let ty = self.ty.to_token_stream(target_ty, crate_location);
        quote! {
            #(#attrs)* #vis #ident #colon_token #ty
        }
    }
}

impl SyntaxFieldType {
    pub fn to_token_stream(&self, target_ty: &Type, crate_location: CrateLocation) -> TokenStream {
        match self {
            SyntaxFieldType::Type(ty) => {
                quote! {#ty}
            }
            SyntaxFieldType::Impl(kind) => kind.to_token_stream(target_ty, crate_location),
        }
    }
}

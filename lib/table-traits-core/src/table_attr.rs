use syn::parse::Parse;
use syn::Token;

#[derive(Debug, Default)]
pub struct TableAttr {
    pub id: bool,
}

impl Parse for TableAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut is_not_first = false;
        let mut data = TableAttr::default();
        while !input.is_empty() {
            if is_not_first {
                input.parse::<Token![,]>()?;
            } else {
                is_not_first = true;
            }
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().as_str() {
                "id" => {
                    data.id = true;
                }
                _ => {
                    return Err(syn::Error::new(ident.span(), "unknown attribute"));
                }
            }
        }
        Ok(data)
    }
}

use proc_macro2::{Ident, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterKind {
    In,
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
    NotIn,
    Like,
    NLike,
    Null,
    NotNull,
    Between,
    NBetween,
    Prefix,
    NPrefix,
    Suffix,
    NSuffix,
    Contain,
    NContain,
    Regex,
}

impl FilterKind {
    pub fn all() -> Vec<FilterKind> {
        vec![
            FilterKind::Eq,
            FilterKind::Ne,
            FilterKind::Gt,
            FilterKind::Lt,
            FilterKind::Gte,
            FilterKind::Lte,
            FilterKind::In,
            FilterKind::NotIn,
            FilterKind::Like,
            FilterKind::NLike,
            FilterKind::Null,
            FilterKind::NotNull,
            FilterKind::Between,
            FilterKind::NBetween,
            FilterKind::Prefix,
            FilterKind::NPrefix,
            FilterKind::Suffix,
            FilterKind::NSuffix,
            FilterKind::Contain,
            FilterKind::NContain,
            FilterKind::Regex,
        ]
    }
    pub fn field_ident(&self, span: Span) -> Ident {
        match self {
            FilterKind::Eq => Ident::new("eq", span),
            FilterKind::Ne => Ident::new("ne", span),
            FilterKind::Gt => Ident::new("gt", span),
            FilterKind::Lt => Ident::new("lt", span),
            FilterKind::Gte => Ident::new("gte", span),
            FilterKind::Lte => Ident::new("lte", span),
            FilterKind::In => Ident::new_raw("in", span),
            FilterKind::NotIn => Ident::new("not_in", span),
            FilterKind::Like => Ident::new("like", span),
            FilterKind::NLike => Ident::new("nlike", span),
            FilterKind::Null => Ident::new("null", span),
            FilterKind::NotNull => Ident::new("not_null", span),
            FilterKind::Between => Ident::new("between", span),
            FilterKind::NBetween => Ident::new("nbetween", span),
            FilterKind::Prefix => Ident::new("prefix", span),
            FilterKind::NPrefix => Ident::new("nprefix", span),
            FilterKind::Suffix => Ident::new("suffix", span),
            FilterKind::NSuffix => Ident::new("nsuffix", span),
            FilterKind::Contain => Ident::new("contain", span),
            FilterKind::NContain => Ident::new("ncontain", span),
            FilterKind::Regex => Ident::new("regex", span),
        }
    }
    pub fn enum_value(&self, span: Span) -> Ident {
        match self {
            FilterKind::Eq => Ident::new("Eq", span),
            FilterKind::Ne => Ident::new("Ne", span),
            FilterKind::Gt => Ident::new("Gt", span),
            FilterKind::Lt => Ident::new("Lt", span),
            FilterKind::Gte => Ident::new("Gte", span),
            FilterKind::Lte => Ident::new("Lte", span),
            FilterKind::In => Ident::new("In", span),
            FilterKind::NotIn => Ident::new("NotIn", span),
            FilterKind::Like => Ident::new("Like", span),
            FilterKind::NLike => Ident::new("NLike", span),
            FilterKind::Null => Ident::new("Null", span),
            FilterKind::NotNull => Ident::new("NotNull", span),
            FilterKind::Between => Ident::new("Between", span),
            FilterKind::NBetween => Ident::new("NBetween", span),
            FilterKind::Prefix => Ident::new("Prefix", span),
            FilterKind::NPrefix => Ident::new("NPrefix", span),
            FilterKind::Suffix => Ident::new("Suffix", span),
            FilterKind::NSuffix => Ident::new("NSuffix", span),
            FilterKind::Contain => Ident::new("Contain", span),
            FilterKind::NContain => Ident::new("NContain", span),
            FilterKind::Regex => Ident::new("Regex", span),
        }
    }
}

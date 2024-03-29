#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterKind {
    // equality
    Eq,
    Ne,
    // comparison
    Gt,
    Lt,
    Gte,
    Lte,
    // set
    In,
    Nin,
    // like
    Like,
    Nlike,
    // null
    Null,
    Nonnull,
    // between
    Between,
    NBetween,
    // string matches
    Prefix,
    NPrefix,
    Suffix,
    NSuffix,
    Contains,
    NContains,
    // regex
    Regex,
}
#[derive(Debug, Clone)]
pub enum FilterValue<T> {
    // equality
    Eq(T),
    Ne(T),
    // comparison
    Gt(T),
    Lt(T),
    Gte(T),
    Lte(T),
    // set
    In(Vec<T>),
    Nin(Vec<T>),
    // like
    Like(T),
    Nlike(T),
    // null
    Null,
    Nonnull,
    // between
    Between(T, T),
    NBetween(T, T),
    // string matches
    Prefix(T),
    NPrefix(T),
    Suffix(T),
    NSuffix(T),
    Contains(T),
    NContains(T),
    // regex
    Regex(T),

    None,
    NotImplemented(FilterKind),
}
pub trait Filter {
    type Target;

    fn implemented_filters() -> Vec<FilterKind>;

    fn activated_filters(&self) -> Vec<FilterKind>;

    fn filter_value(&self, kind: FilterKind) -> FilterValue<Self::Target>;
}

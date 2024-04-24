use sea_query::{Condition, IntoColumnRef};

use crate::private::FilterType;

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
    NotIn,
    // like
    Like,
    NLike,
    // null
    Null,
    NotNull,
    // between
    Between,
    NBetween,
    // string matches
    Prefix,
    NPrefix,
    Suffix,
    NSuffix,
    Contain,
    NContain,
    // regex
    Regex,
    // TODO : json 추가하기
    // json
    // JsonpathContain,
    // JsonpathEqual,
}

pub trait Filter {
    type Target: FilterType;

    fn build_condition(&self, target_column: impl IntoColumnRef + Clone) -> Option<Condition>;
}

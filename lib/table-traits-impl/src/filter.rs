use sea_query::{ColumnRef, Condition, IntoColumnRef};

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

    fn implemented_filters() -> Vec<FilterKind>;

    fn build_condition(&self, filter_kind: FilterKind, target_column: impl IntoColumnRef + Clone) -> Option<Condition>;

    fn build_all_condition(&self, target_column: impl IntoColumnRef + Clone) -> Condition {
        let activated = Self::implemented_filters();
        let target_column = target_column.into_column_ref();
        let mut condition = Condition::all();
        for kind in activated {
            condition = condition.add_option(self.build_condition(kind, target_column.clone()));
        }
        condition
    }
}

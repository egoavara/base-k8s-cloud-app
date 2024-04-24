use sea_query::{Condition, IntoColumnRef, NullOrdering, Order, SimpleExpr};

pub trait Sorter {
    type Target;

    fn build_order(
        &self,
        target_column: impl IntoColumnRef + Clone,
    ) -> (SimpleExpr, Order, Option<NullOrdering>);

    fn build_equal(
        &self,
        value: Self::Target,
        target_column: impl IntoColumnRef + Clone,
    ) -> Condition;

    fn build_after(
        &self,
        value: Self::Target,
        target_column: impl IntoColumnRef + Clone,
    ) -> Condition;

    fn build_before(
        &self,
        value: Self::Target,
        target_column: impl IntoColumnRef + Clone,
    ) -> Condition;
}

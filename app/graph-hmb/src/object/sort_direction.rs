use async_graphql::Enum;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Enum)]
pub enum SortDirection {
    Ascending,
    Descending,
}
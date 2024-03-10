use async_graphql::{Enum, InputObject};

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Enum)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl From<SortDirection> for sea_query::Order {
    fn from(src: SortDirection) -> Self {
        match src {
            SortDirection::Ascending => sea_query::Order::Asc,
            SortDirection::Descending => sea_query::Order::Desc,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, InputObject)]
pub struct AscendingOption {
    pub order: u16,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, InputObject)]
pub struct SortOption {
    pub order: u16,
    pub direction: SortDirection,
}

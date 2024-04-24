use async_graphql::Enum;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Enum)]
pub enum OrderBoth {
    Asc,
    Desc,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Enum)]
pub enum OrderAscOnly {
    Asc,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Enum)]
pub enum OrderDescOnly {
    Desc,
}

use async_graphql::{InputObject};

#[derive(Debug, Clone, InputObject)]
pub struct Cursor {
    pub after: Option<String>,
    pub first: Option<usize>,
    pub before: Option<String>,
    pub last: Option<usize>,
}
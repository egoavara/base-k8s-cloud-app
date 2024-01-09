use async_graphql::InputObject;

#[derive(Debug, Clone, Default, InputObject)]
pub struct UuidFilter {
    pub eq: Option<String>,
    pub r#in: Option<Vec<String>>,
}
use async_graphql::InputObject;

#[derive(Debug, Clone, Default, InputObject)]
pub struct NameFilter {
    pub eq: Option<String>,
    pub ne: Option<String>,

    pub r#in: Option<Vec<String>>,
    pub nin: Option<Vec<String>>,

    pub contains: Option<String>,
    pub ncontains: Option<String>,
    pub starts_with: Option<String>,
    pub nstarts_with: Option<String>,

    pub like: Option<String>,
    pub nlike: Option<String>,

    pub query: Option<String>,
}
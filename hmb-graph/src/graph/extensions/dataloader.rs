use std::collections::HashMap;
use std::hash::Hash;
use async_graphql::dataloader::Loader;

#[async_trait::async_trait]
pub trait LoadableEntity<ID : Send + Sync + Hash + Eq + Clone + 'static, T>{
    type Loader: Loader<Self>;
    type Error;

    async fn load(&self, keys: &[ID]) -> Result<HashMap<ID, T>, Self::Error>;
}
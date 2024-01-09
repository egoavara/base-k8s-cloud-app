use std::hash::Hash;

use async_graphql::dataloader::Loader;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait GeneralTable: Sized + Send + Sync + 'static {
    type Identity;
    type Key: Send + Sync + Hash + Eq + Clone + 'static;
    type Loader: Loader<Self::Key>;
    type Filter;
    type Sorting;

    fn loader(pool: &PgPool) -> Self::Loader;

    async fn load(loader: &Self::Loader, filter: Self::Filter, sorting: Self::Sorting) -> Result<Vec<Self>, anyhow::Error>;
}
use crate::entity;

use async_graphql::connection::Connection;
use async_graphql::dataloader::{DataLoader, Loader};
use async_graphql::{ComplexObject, Context, Object, SimpleObject};

use general_table::object::Cursor;
use general_table::traits::TableDefinition;

use std::collections::HashMap;
use std::convert::Infallible;
use tracing::instrument;


pub struct Query;

#[Object]
impl Query {
    #[instrument(level = "info", name = "find_otype", skip_all)]
    #[graphql(complexity = 5)]
    async fn find_otype<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "Cursor for pagination")] cursor: Option<Cursor>,
        #[graphql(desc = "Filter for the query")] filter: Option<
            <entity::public::Otype as TableDefinition>::Filter,
        >,
        #[graphql(desc = "Sorting for the query")] sorting: Option<
            <entity::public::Otype as TableDefinition>::Sorting,
        >,
    ) -> Connection<String, entity::public::Otype> {
        entity::public::Otype::find(
            ctx,
            cursor.unwrap_or_default(),
            filter.unwrap_or_default(),
            sorting.unwrap_or_default(),
        )
        .await
        .unwrap()
    }
    #[instrument(level = "info", name = "find_otype", skip_all)]
    async fn find_project<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "Cursor for pagination")] cursor: Option<Cursor>,
        #[graphql(desc = "Filter for the query")] filter: Option<
            <entity::public::Project as TableDefinition>::Filter,
        >,
        #[graphql(desc = "Sorting for the query")] sorting: Option<
            <entity::public::Project as TableDefinition>::Sorting,
        >,
    ) -> Connection<String, entity::public::Project> {
        entity::public::Project::find(
            ctx,
            cursor.unwrap_or_default(),
            filter.unwrap_or_default(),
            sorting.unwrap_or_default(),
        )
        .await
        .unwrap()
    }

    async fn find_test<'ctx>(&self, _ctx: &Context<'ctx>, names: Vec<String>) -> Vec<Test> {
        names.into_iter().map(|name| Test { name }).collect()
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Test {
    pub name: String,
}

#[ComplexObject]
impl Test {
    async fn test<'ctx>(&self, ctx: &Context<'ctx>, names: Vec<String>) -> Vec<Inner> {
        let loader = ctx.data_unchecked::<DataLoader<InnerLoader>>();
        let result = loader.load_many(names).await.unwrap();
        result.into_values().collect()
    }
}

#[derive(SimpleObject, Clone)]
pub struct Inner {
    pub name: String,
}

pub struct InnerLoader;

#[async_trait::async_trait]
impl Loader<String> for InnerLoader {
    type Value = Inner;
    type Error = Infallible;

    async fn load(&self, keys: &[String]) -> Result<HashMap<String, Self::Value>, Self::Error> {
        println!("load: {:?}", keys);
        let mut map = HashMap::new();
        for key in keys {
            map.insert(key.clone(), Inner { name: key.clone() });
        }
        Ok(map)
    }
}

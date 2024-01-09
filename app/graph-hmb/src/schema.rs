use async_graphql::{Context, Object};
use async_graphql::dataloader::Loader;
use sqlx::query_as;
use tracing::{info, instrument};

use crate::entity;
use crate::entity::Story;
use crate::object::Cursor;
use crate::traits::GeneralTable;

pub type Schema = async_graphql::Schema<Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub struct Query;


#[Object]
impl Query {
    #[instrument(level = "info", name = "hero", skip_all)]
    async fn hero<'a>(
        &self,
        ctx: &Context<'a>,
        // #[graphql(
        // // desc = "If omitted, returns the hero of the whole saga. If provided, returns the hero of that particular episode."
        // )]
        // episode: Option<>,
    ) -> entity::Story {
        let pool = ctx.data::<sqlx::PgPool>().unwrap();
        let story = query_as!(Story, r#"select * from hmb.story where story_id = '018835bd-ba8c-7078-ba9e-8c4e85604870'"#)
            .fetch_one(pool)
            .await
            .unwrap();
        story.into()
    }

    #[instrument(level = "info", name = "stories", skip_all)]
    async fn find_stories<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(desc = "The cursor for pagination")]
        cursor: Option<Cursor>,
        filter: Option<<Story as GeneralTable>::Filter>,
        sorting: Option<<Story as GeneralTable>::Sorting>,
    ) -> Vec<entity::Story> {
        let loader = ctx.data::<<Story as GeneralTable>::Loader>().unwrap();
        let result = Story::load(&loader, filter.unwrap_or_default(), sorting.unwrap_or_default())
            .await
            .unwrap();
        result
    }
}
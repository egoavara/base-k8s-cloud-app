use async_graphql::connection::Connection;
use async_graphql::{Context, Object};
use tracing::instrument;

use general_table::object::Cursor;
use general_table::traits::TableDefinition;

use crate::entity;

pub type Schema =
    async_graphql::Schema<Query, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    // #[instrument(level = "info", name = "hero", skip_all)]
    // async fn hero<'a>(
    //     &self,
    //     ctx: &Context<'a>,
    //     // #[graphql(
    //     // // desc = "If omitted, returns the hero of the whole saga. If provided, returns the hero of that particular episode."
    //     // )]
    //     // episode: Option<>,
    // ) -> entity::Story {
    //     let pool = ctx.data::<sqlx::PgPool>().unwrap();
    //     let story = query_as!(Story, r#"select * from hmb.story where story_id = '018835bd-ba8c-7078-ba9e-8c4e85604870'"#)
    //         .fetch_one(pool)
    //         .await
    //         .unwrap();
    //     story.into()
    // }

    #[instrument(level = "info", name = "find_otype", skip_all)]
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
        let result = entity::public::Otype::find(
            ctx,
            cursor.unwrap_or_default(),
            filter.unwrap_or_default(),
            sorting.unwrap_or_default(),
        )
        .await
        .unwrap();
        result
    }
}

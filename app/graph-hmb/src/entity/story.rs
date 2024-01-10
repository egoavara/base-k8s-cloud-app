use std::collections::HashMap;
use std::convert::Infallible;

use async_graphql::{InputObject, SimpleObject};
use async_graphql::async_trait::async_trait;
use async_graphql::dataloader::Loader;
use sea_query::{ColumnRef, Expr, IdenStatic, PgFunc, PostgresQueryBuilder, Query};
use sea_query::extension::postgres::PgExpr;
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, query_as};
use sqlx::types::time::OffsetDateTime;
use sqlx::types::Uuid;
use tracing::{info, instrument};

use graph_guard::FieldGuard;
use crate::entity::Hmb;

use crate::object::{NameFilter, OffsetDateTimeFilter, SortDirection, UuidFilter};
use crate::traits::GeneralTable;
use graph_guard::rebac;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, SimpleObject)]
pub struct Story {
    // #[graphql(guard = r#"FieldGuard::new("Story", "story_id")"#)]
    #[graphql(directive = rebac::apply("allow".to_string(), "Story".to_string(), "story_id".to_string(), true))]
    pub story_id: Uuid,
    // #[graphql(guard = r#"FieldGuard::new("Story", "title")"#)]
    pub title: String,
    // #[graphql(guard = r#"FieldGuard::new("Story", "post_at")"#)]
    pub post_at: OffsetDateTime,
}

pub struct StoryLoader(PgPool);

#[derive(Debug, Default, InputObject)]
pub struct StoryFilter {
    story_id: Option<UuidFilter>,
    title: Option<NameFilter>,
    post_at: Option<OffsetDateTimeFilter>,
}

#[derive(IdenStatic, Copy, Clone)]
#[iden = "story"]
pub enum StoryIdentity {
    Table,
    StoryId,
    Title,
    PostAt,
}

#[derive(Debug, Default, InputObject)]
pub struct StorySorting {
    post_at: Option<SortDirection>,
}

#[async_trait]
impl GeneralTable for Story {
    type Identity = StoryIdentity;
    type Key = Uuid;
    type Loader = StoryLoader;
    type Filter = StoryFilter;
    type Sorting = StorySorting;

    fn loader(pool: &PgPool) -> Self::Loader {
        StoryLoader(pool.clone())
    }
    #[instrument(skip_all)]
    async fn load(loader: &Self::Loader, filter: Self::Filter, sorting: Self::Sorting) -> Result<Vec<Self>, anyhow::Error> {
        let UuidFilter {
            eq: story_id_eq,
            r#in: story_id_in,
        } = filter.story_id
            .unwrap_or(UuidFilter::default());
        let NameFilter {
            eq: name_eq,
            ne: name_ne,
            r#in: name_in,
            nin: name_not_in,
            starts_with: name_starts_with,
            nstarts_with: name_nstarts_with,
            contains: name_contains,
            ncontains: name_ncontains,
            like: name_like,
            nlike: name_nlike,
            query: name_query,
        } = filter.title
            .unwrap_or(NameFilter::default());
        let OffsetDateTimeFilter {
            eq: post_at_eq,
            ne: post_at_ne,
            gt: post_at_gt,
            gte: post_at_gte,
            lt: post_at_lt,
            lte: post_at_lte,
        } = filter.post_at
            .unwrap_or(OffsetDateTimeFilter::default());

        let (query, values) = Query::select()
            .column(ColumnRef::Asterisk)
            .from((Hmb, StoryIdentity::Table))
            .and_where_option(story_id_eq.map(|eq| Expr::col(StoryIdentity::StoryId).eq(eq)))
            .and_where_option(story_id_in.map(|r#in| Expr::col(StoryIdentity::StoryId).in_tuples(r#in)))
            .and_where_option(name_eq.map(|eq| Expr::col(StoryIdentity::Title).eq(eq)))
            .and_where_option(name_ne.map(|ne| Expr::col(StoryIdentity::Title).eq(Expr::val(ne).not())))
            .and_where_option(name_in.map(|r#in| Expr::col(StoryIdentity::Title).in_tuples(r#in)))
            .and_where_option(name_not_in.map(|nin| Expr::col(StoryIdentity::Title).in_tuples(nin).not()))
            .and_where_option(name_starts_with.map(|starts_with| Expr::value(PgFunc::starts_with(Expr::col(StoryIdentity::Title), Expr::val(starts_with)))))
            .and_where_option(name_nstarts_with.map(|nstarts_with| Expr::value(PgFunc::starts_with(Expr::col(StoryIdentity::Title), Expr::val(nstarts_with))).not()))
            .and_where_option(name_contains.map(|contains| Expr::col(StoryIdentity::Title).contains(contains)))
            .and_where_option(name_ncontains.map(|ncontains| Expr::col(StoryIdentity::Title).contains(ncontains).not()))
            .and_where_option(name_like.map(|like| Expr::col(StoryIdentity::Title).like(like)))
            .and_where_option(name_nlike.map(|nlike| Expr::col(StoryIdentity::Title).like(nlike).not()))
            // TODO : query
            .and_where_option(post_at_eq.map(|eq| Expr::col(StoryIdentity::PostAt).eq(eq)))
            .and_where_option(post_at_ne.map(|ne| Expr::col(StoryIdentity::PostAt).eq(ne).not()))
            .and_where_option(post_at_gt.map(|gt| Expr::col(StoryIdentity::PostAt).gt(gt)))
            .and_where_option(post_at_gte.map(|gte| Expr::col(StoryIdentity::PostAt).gte(gte)))
            .and_where_option(post_at_lt.map(|lt| Expr::col(StoryIdentity::PostAt).lt(lt)))
            .and_where_option(post_at_lte.map(|lte| Expr::col(StoryIdentity::PostAt).lte(lte)))
            .limit(2)
            .build_sqlx(PostgresQueryBuilder);
        let result = sqlx::query_as_with::<_, Story, _>(&query, values)
            .fetch_all(&loader.0)
            .await
            .map_err(|err| anyhow::anyhow!(err))?;
        Ok(result)
    }
}


#[async_trait]
impl Loader<Uuid> for StoryLoader {
    type Value = Story;
    type Error = Infallible;
    #[instrument(skip_all)]
    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let result = query_as!(Story,
            r#"
                select  *
                from    hmb.story
                where   story_id = any( $1 )
            "#,
            keys
        )
            .fetch_all(&self.0)
            .await
            .unwrap();

        Ok(result.into_iter().map(|story| (story.story_id, story)).collect())
    }
}
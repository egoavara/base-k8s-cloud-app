use std::borrow::Cow;
use std::collections::HashMap;

use async_graphql::connection::Connection;
use async_graphql::futures_util::FutureExt;
use async_graphql::{registry, Context, Enum, InputObject, InputType, InputValueError, InputValueResult, Object, OneofObject, SimpleObject, Union, Value};
use sea_query::{Alias, Asterisk, Cond, NullOrdering, OrderExpr, OrderedStatement, SelectStatement, SimpleExpr, TableRef};
use sea_query_binder::SqlxBinder;
use sqlx::{Executor, FromRow, PgPool};
use tracing::{info, instrument};
use tracing_subscriber::fmt::format;
use uuid::Uuid;

use table_traits::types::DEFAULT_CONFIG;
use table_traits::{Cursor, CursorWrap, Field, Page, PageByCursor, Table, TableFilter, TableSorter, UuidFilter};

#[derive(SimpleObject)]
// #[derive(SimpleObject, Table)]
#[derive(Table, FromRow)]
#[table(schema = "public", table = "otype")]
pub struct TableEx {
    #[column(id, filter, sorter)]
    pub otype_id: Uuid,
    #[column(filter, sorter)]
    pub name: String,
    #[column()]
    pub description: Option<String>,
    #[column()]
    pub definition: serde_json::Value,
}

pub struct Query;

#[Object]
impl Query {
    #[instrument(level = "info", name = "test", skip_all)]
    async fn test<'ctx>(&self, ctx: &Context<'ctx>, page: Option<PageByCursor<TableExCursor>>, filter: TableExFilter, sorter: TableExSorter) -> Connection<CursorWrap<TableExCursor>, TableEx> {
        let pool = ctx.data_unchecked::<PgPool>();
        let page = page.map(|x| Page::from(x));
        let result = DEFAULT_CONFIG
            .context_as(Some("hello".to_string()))
            .connection(
                page,
                filter,
                sorter,
                move |c| {
                    info!("c : {:?}", c);
                    async { None }
                },
                move |query| {
                    let (q, v) = query.build_sqlx(sea_query::PostgresQueryBuilder);
                    info!("q : {}", q);
                    info!("v : {:?}", v);
                    async move {
                        let result = sqlx::query_scalar_with::<_, Uuid, _>(&q, v).fetch_all(pool).await.unwrap();
                        result
                    }
                },
                move |ids| {
                    info!("ids : {:?}", ids);
                    async move {
                        let a = sqlx::query_as!(TableEx, "select * from otype where otype_id = any($1)", &ids).fetch_all(pool).await.unwrap();
                        a.into_iter().map(|x| (x.otype_id, x)).collect::<HashMap<_, _>>()
                    }
                },
            )
            .await;
        result.unwrap()
    }
    // #[instrument(level = "info", name = "test", skip_all)]
    // async fn test_by_id<'ctx>(&self, ctx: &Context<'ctx>, id: Uuid) -> Test {
    //     let filter = TestFilter::by_id(id);
    //     let cond = filter.to_condition();
    //     OrderedStatement::order_by_expr_with_nulls()
    //     let (q, v) = sea_query::Query::select()
    //         .order_by_expr_with_nulls()
    //         .column(Asterisk)
    //         .from(Alias::new("test"))
    //         .cond_where(cond)
    //         .order_by_expr_with_nulls()
    //         .build(sea_query::PostgresQueryBuilder);
    //     info!("q : {}", q);
    //     info!("v : {:?}", v);
    //     Test {
    //         id: Uuid::new_v4(),
    //         name: "test".to_string(),
    //         age: 10,
    //     }
    // }
}

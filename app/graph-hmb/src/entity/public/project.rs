use std::collections::HashMap;
use std::convert::Infallible;

use async_graphql::connection::{Connection, Edge, EmptyFields};
use async_graphql::dataloader::{DataLoader, Loader};
use async_graphql::futures_util::future::join_all;
use async_graphql::futures_util::FutureExt;
use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use async_trait::async_trait;
use itertools::Itertools;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::Uuid;
use sqlx::{query_as, FromRow, PgPool, Row};
use tap::{Tap, TapOptional};
use tracing::instrument;

use crate::entity::public::{Otype, OtypeFilter, OtypeLoader};
use general_table::config::GeneralTableConfig;

use general_table::object::{Cursor, IdRef, NameFilter, UuidFilter};
use general_table::scalar::{AscendingOption, SortDirection, SortOption};
use general_table::traits::{FilterField, FilterTable, SortingTable, TableDefinition};

#[enum_def(prefix = "", suffix = "Refs")]
#[derive(Serialize, Deserialize, Debug, Clone, FromRow, SimpleObject)]
#[graphql(complex)]
pub struct Project {
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[ComplexObject]
impl Project {
    #[instrument(skip_all)]
    pub async fn otype<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        filter: Option<OtypeFilter>,
    ) -> async_graphql::Result<Vec<Otype>> {
        let loader = ctx.data_unchecked::<DataLoader<OtypeLoader>>();
        let id = IdRef::<Project, Otype>::group_by_filter(ctx.path_node, self.project_id, filter);
        let result: Vec<Otype> = loader.load_one(id).await?.unwrap();
        Ok(result)
    }
    pub async fn set(&self, hello: String) -> bool {
        println!("{}", hello);
        false
    }
}

pub struct ProjectLoader(PgPool);

#[derive(Debug, Default, Clone, InputObject)]
pub struct ProjectFilter {
    pub project_id: Option<UuidFilter>,
    pub name: Option<NameFilter>,
}

#[derive(Debug, Default, InputObject)]
pub struct ProjectSorting {
    pub project_id: Option<AscendingOption>,
    pub name: Option<SortOption>,
}

impl FilterTable for ProjectFilter {
    type Table = Project;

    fn where_clause(&self) -> Vec<Option<SimpleExpr>> {
        vec![
            self.project_id
                .as_ref()
                .and_then(|x| x.to_expr(ProjectRefs::ProjectId)),
            self.name
                .as_ref()
                .and_then(|x| x.to_expr(ProjectRefs::Name)),
        ]
    }
}

impl SortingTable for ProjectSorting {
    type Table = Project;

    fn order_by_clause(
        &self,
    ) -> Vec<(<Self::Table as TableDefinition>::References, SortDirection)> {
        vec![
            self.project_id
                .map(|x| (ProjectRefs::ProjectId, x.order, SortDirection::Ascending)),
            self.name.map(|x| (ProjectRefs::Name, x.order, x.direction)),
        ]
        .into_iter()
        .flatten()
        .sorted_by(|(_, x, _), (_, y, _)| x.cmp(y))
        .map(|(x, _, y)| (x, y))
        .collect()
    }
}

#[async_trait]
impl TableDefinition for Project {
    type Id = Uuid;
    type References = ProjectRefs;
    type Loader = ProjectLoader;
    type Filter = ProjectFilter;
    type Sorting = ProjectSorting;

    fn table() -> Self::References {
        ProjectRefs::Table
    }

    fn id_column() -> Self::References {
        ProjectRefs::ProjectId
    }

    fn encode_field(&self, key: Self::References) -> Value {
        match key {
            ProjectRefs::ProjectId => serde_json::to_value(self.project_id).unwrap(),
            ProjectRefs::Name => serde_json::to_value(self.name.clone()).unwrap(),
            ProjectRefs::Description => serde_json::to_value(self.description.clone()).unwrap(),
            _ => unreachable!(),
        }
    }

    fn decode_field(key: Self::References, value: Value) -> SimpleExpr {
        match key {
            ProjectRefs::ProjectId => serde_json::from_value::<Uuid>(value)
                .map(Expr::value)
                .unwrap(),
            ProjectRefs::Name => serde_json::from_value::<String>(value)
                .map(Expr::value)
                .unwrap(),
            ProjectRefs::Description => serde_json::from_value::<String>(value)
                .map(Expr::value)
                .unwrap(),
            _ => unreachable!(),
        }
    }

    #[instrument(skip_all)]
    async fn find<'a>(
        ctx: &Context<'a>,
        cursor: Cursor,
        filter: Self::Filter,
        sorting: Self::Sorting,
    ) -> Result<Connection<String, Self, EmptyFields, EmptyFields>, async_graphql::Error> {
        // 필요한 데이터 로더 및 설정 정보 가져오기
        let config = ctx.data_unchecked::<GeneralTableConfig>();
        let pool = ctx.data_unchecked::<PgPool>();
        let loader = ctx.data_unchecked::<DataLoader<Self::Loader>>();
        // 커서 값 검증
        let cursor = cursor.validate(&config.cursor_config, &sorting)?;
        // select 문 동적 생성 시작
        let (query, parames) = Query::select()
            // 필요한 컬럼 선택
            .tap_mut(|x| {
                x.column(ProjectRefs::ProjectId).from(ProjectRefs::Table);
            })
            // where 절 동적 생성 시작
            // - 필터 적용
            .tap_mut(|x| {
                filter.apply(x);
            })
            // - 페이지네이션 (커서 기반) 에 필요한 조건문 적용
            .tap_mut(|x| {
                cursor.apply(x, &sorting);
            })
            // order by 정렬 수행
            .tap_mut(|x| {
                sorting.apply(x);
            })
            // limit 적용
            .tap_mut(|x| {
                cursor.limit.tap_some(|limit| {
                    x.limit(*limit as u64);
                });
            })
            .build_sqlx(PostgresQueryBuilder);
        // 대상 id 전체 조회
        let result_ids = sqlx::query_scalar_with::<_, Uuid, _>(&query, parames)
            .fetch_all(pool)
            .await?;
        let result = join_all(result_ids.into_iter().map(|id| {
            FutureExt::map(loader.load_one(id), |x| x.unwrap().unwrap())
                .map(|x| Edge::new(sorting.encode_key(&x), x))
        }))
        .await;
        // has_next, has_prev 계산
        let (cursor_query, cursor_values) = cursor
            .prepare_cursor_result(&result, &filter, &sorting)
            .build_sqlx(PostgresQueryBuilder);
        let row = sqlx::query_with(&cursor_query, cursor_values)
            .fetch_one(pool)
            .await?;
        let has_prev = row.get::<bool, _>(0);
        let has_next = row.get::<bool, _>(1);
        // 결과물 반환
        let mut connection = Connection::new(has_prev, has_next);
        connection.edges.extend(result);
        Ok(connection)
    }
}

#[async_trait]
impl Loader<Uuid> for ProjectLoader {
    type Value = Project;
    type Error = Infallible;
    #[instrument(skip_all)]
    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let result = query_as!(
            Project,
            r#"
                select  *
                from    public.project
                where   project_id = any($1)
            "#,
            keys
        )
        .fetch_all(&self.0)
        .await
        .unwrap();

        Ok(result
            .into_iter()
            .map(|val| (val.project_id, val))
            .collect())
    }
}

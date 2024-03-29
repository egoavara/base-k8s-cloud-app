use std::collections::HashMap;
use std::convert::Infallible;
use std::hash::Hash;

use async_graphql::async_trait::async_trait;
use async_graphql::connection::{Connection, Edge, EmptyFields};
use async_graphql::dataloader::{DataLoader, Loader};

use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use futures::future::{join_all, FutureExt};
use itertools::Itertools;
use sea_query::{
    enum_def, ColumnRef, Expr, JoinType, PostgresQueryBuilder, Query, SeaRc, SimpleExpr,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::Uuid;
use sqlx::{query_as, FromRow, PgPool, Row};
use tap::{Tap, TapOptional};
use tracing::{info, instrument};

use general_table::config::GeneralTableConfig;
use general_table::object::{Cursor, IdRef, NameFilter, UuidFilter};
use general_table::scalar::{AscendingOption, SortDirection, SortOption};
use general_table::traits::{
    FilterField, FilterTable, InsertingError, InsertingTable, SortingTable, TableDefinition,
    UpdatingTable,
};
use graph_guard::rebac;

use crate::entity::public::{Project, ProjectOtypeRefs};

#[enum_def(prefix = "", suffix = "Refs")]
#[derive(Serialize, Deserialize, Debug, Clone, FromRow, SimpleObject)]
#[graphql(complex)]
pub struct Otype {
    pub otype_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[graphql(directive = rebac::apply("allow", "field", "Otype.definition"))]
    pub definition: Value,
}

#[ComplexObject]
impl Otype {}

pub struct OtypeLoader(PgPool);

#[derive(Debug, Default, Clone, InputObject)]
pub struct OtypeFilter {
    pub otype_id: Option<UuidFilter>,
    pub name: Option<NameFilter>,
}

#[derive(Debug, Default, Clone, InputObject)]
pub struct OtypeSorting {
    pub otype_id: Option<AscendingOption>,
    pub name: Option<SortOption>,
}

#[derive(Debug, Default, Clone, SimpleObject)]
pub struct OtypeModifing {
    pub name: Option<String>,
    pub description: Option<String>,
    #[graphql(directive = rebac::apply("allow", "field", "Otype.definition"))]
    pub definition: Option<Value>,
}

#[derive(Debug, Default, Clone, InputObject)]
pub struct OtypeCreating {
    pub name: String,
    pub description: Option<String>,
    pub definition: Option<Value>,
}

impl FilterTable for OtypeFilter {
    type Table = Otype;

    fn where_clause(&self) -> Vec<Option<SimpleExpr>> {
        vec![
            self.otype_id
                .as_ref()
                .and_then(|x| x.to_expr(OtypeRefs::OtypeId)),
            self.name.as_ref().and_then(|x| x.to_expr(OtypeRefs::Name)),
        ]
    }
}

impl SortingTable for OtypeSorting {
    type Table = Otype;

    fn order_by_clause(
        &self,
    ) -> Vec<(<Self::Table as TableDefinition>::References, SortDirection)> {
        vec![
            self.otype_id
                .map(|x| (OtypeRefs::OtypeId, x.order, SortDirection::Ascending)),
            self.name.map(|x| (OtypeRefs::Name, x.order, x.direction)),
        ]
        .into_iter()
        .flatten()
        .sorted_by(|(_, x, _), (_, y, _)| x.cmp(y))
        .map(|(x, _, y)| (x, y))
        .collect()
    }
}

#[async_trait]
impl InsertingTable for OtypeCreating {
    type Returning = Otype;
    type Failure = String;

    async fn insert<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Self::Returning, InsertingError<Self::Failure>> {
        let pool = ctx.data_unchecked::<PgPool>();
        let row = sqlx::query_as!(
            Otype,
            r#"
                insert into public.otype (name, description, definition)
                values ($1, $2, $3)
                returning *
            "#,
            self.name,
            self.description,
            self.definition
        )
        .fetch_one(pool)
        .await
        .map_err(|err| InsertingError::Throw(err.to_string()))?;
        Ok(row)
    }
}
//
// impl UpdatingTable for OtypeModifing {
//     type Table = Otype;
// }

#[async_trait]
impl TableDefinition for Otype {
    type Id = Uuid;
    type References = OtypeRefs;
    type Loader = OtypeLoader;
    type Filter = OtypeFilter;
    type Sorting = OtypeSorting;

    fn table() -> Self::References {
        OtypeRefs::Table
    }

    fn id_column() -> Self::References {
        OtypeRefs::OtypeId
    }

    fn encode_field(&self, key: Self::References) -> Value {
        match key {
            OtypeRefs::OtypeId => serde_json::to_value(self.otype_id).unwrap(),
            OtypeRefs::Name => serde_json::to_value(self.name.clone()).unwrap(),
            OtypeRefs::Description => serde_json::to_value(self.description.clone()).unwrap(),
            OtypeRefs::Definition => self.definition.clone(),
            _ => unreachable!(),
        }
    }

    fn decode_field(key: Self::References, value: Value) -> SimpleExpr {
        match key {
            OtypeRefs::OtypeId => serde_json::from_value::<Uuid>(value)
                .map(Expr::value)
                .unwrap(),
            OtypeRefs::Name => serde_json::from_value::<String>(value)
                .map(Expr::value)
                .unwrap(),
            OtypeRefs::Description => serde_json::from_value::<String>(value)
                .map(Expr::value)
                .unwrap(),
            OtypeRefs::Definition => Expr::value(value),
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
                x.column(OtypeRefs::OtypeId).from(OtypeRefs::Table);
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
        info!("otype find query: {}, {:?}", &query, &parames);
        let result_ids = sqlx::query_scalar_with::<_, Uuid, _>(&query, parames)
            .fetch_all(pool)
            .await?;
        let result = join_all(result_ids.into_iter().map(|otype_id| {
            FutureExt::map(loader.load_one(otype_id), |x| x.unwrap().unwrap())
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
impl Loader<Uuid> for OtypeLoader {
    type Value = Otype;
    type Error = Infallible;
    #[instrument(skip_all)]
    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let result = query_as!(
            Otype,
            r#"
                select  *
                from    public.otype
                where   otype_id = any($1)
            "#,
            keys
        )
        .fetch_all(&self.0)
        .await
        .unwrap();

        Ok(result.into_iter().map(|val| (val.otype_id, val)).collect())
    }
}

#[async_trait]
impl Loader<IdRef<Project, Otype>> for OtypeLoader {
    type Value = Vec<Otype>;
    type Error = Infallible;
    #[instrument(skip_all)]
    async fn load(
        &self,
        keys: &[IdRef<Project, Otype>],
    ) -> Result<HashMap<IdRef<Project, Otype>, Self::Value>, Self::Error> {
        let mut result = HashMap::new();
        let mut grouped_by = HashMap::<Option<String>, (OtypeFilter, Vec<Uuid>)>::new();
        for data in keys {
            result.entry(data.clone()).or_insert_with(Vec::new);
            grouped_by
                .entry(data.group.clone())
                .or_insert_with(|| (data.filter.clone().unwrap_or_default(), Vec::new()))
                .1
                .push(data.id);
        }
        #[derive(FromRow, Debug)]
        struct ProjectOtype {
            project_id: Uuid,
            #[sqlx(flatten)]
            otype: Otype,
        }
        for (group, (filter, data)) in grouped_by.into_iter() {
            // select 문 동적 생성 시작
            let (query, parames) = Query::select()
                // 필요한 컬럼 선택
                .tap_mut(|x| {
                    x.column((ProjectOtypeRefs::Table, ProjectOtypeRefs::ProjectId))
                        .column(ColumnRef::TableAsterisk(SeaRc::new(OtypeRefs::Table)))
                        .from(ProjectOtypeRefs::Table)
                        .join(
                            JoinType::InnerJoin,
                            OtypeRefs::Table,
                            Expr::col((OtypeRefs::Table, OtypeRefs::OtypeId)).eq(Expr::col((
                                ProjectOtypeRefs::Table,
                                ProjectOtypeRefs::OtypeId,
                            ))),
                        );
                })
                // where 절 동적 생성 시작
                // - 필터 적용
                .tap_mut(|x| {
                    filter.apply(x);
                    x.and_where(
                        Expr::col((ProjectOtypeRefs::Table, ProjectOtypeRefs::ProjectId))
                            .is_in(data.clone()),
                    );
                })
                .build_sqlx(PostgresQueryBuilder);
            let temp = sqlx::query_as_with::<_, ProjectOtype, _>(&query, parames)
                .fetch_all(&self.0)
                .await
                .unwrap();
            for data in temp.into_iter() {
                result
                    .get_mut(&IdRef::<Project, Otype>::new(
                        group.clone(),
                        data.project_id,
                        None,
                    ))
                    .unwrap()
                    .push(data.otype);
            }
        }
        Ok(result)
    }
}

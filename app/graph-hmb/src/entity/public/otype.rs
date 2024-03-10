use std::any::Any;
use std::collections::HashMap;
use std::convert::{identity, Infallible};
use std::fmt::Display;
use std::hash::Hash;
use std::iter::Map;

use async_graphql::async_trait::async_trait;
use async_graphql::connection::{Connection, Edge, EmptyFields};
use async_graphql::dataloader::{DataLoader, Loader};
use async_graphql::{Context, InputObject, SimpleObject};
use futures::future::{join_all, BoxFuture, FutureExt};
use itertools::Itertools;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::Uuid;
use sqlx::IntoArguments;
use sqlx::{query_as, Database, Encode, FromRow, PgPool, Postgres, QueryBuilder, Type};
use tap::{Tap, TapOptional};
use tracing::{info, instrument};

use general_table::config::GeneralTableConfig;
use general_table::object::{Cursor, NameFilter, UuidFilter};
use general_table::scalar::{AscendingOption, SortDirection, SortOption};
use general_table::traits::{FilterField, FilterTable, SortingTable, TableDefinition};
use graph_guard::rebac;

#[enum_def(prefix = "", suffix = "Refs")]
#[derive(Serialize, Deserialize, Debug, Clone, FromRow, SimpleObject)]
pub struct Otype {
    pub otype_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[graphql(directive = rebac::apply("allow", "field", "Otype.definition"))]
    pub definition: Value,
}

pub struct OtypeLoader(PgPool);

#[derive(Debug, Default, InputObject)]
pub struct OtypeFilter {
    otype_id: Option<UuidFilter>,
    name: Option<NameFilter>,
}

#[derive(Debug, Default, InputObject)]
pub struct OtypeSorting {
    otype_id: Option<AscendingOption>,
    name: Option<SortOption>,
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
        .filter_map(identity)
        .sorted_by(|(_, x, _), (_, y, _)| x.cmp(y))
        .map(|(x, _, y)| (x, y))
        .collect()
    }

    fn key_candidate(
        &self,
        table: &Self::Table,
    ) -> HashMap<<Self::Table as TableDefinition>::References, Value> {
        let mut result = HashMap::new();
        if self.otype_id.is_some() {
            result.insert(
                OtypeRefs::OtypeId,
                serde_json::to_value(&table.otype_id).unwrap(),
            );
        }
        if self.name.is_some() {
            result.insert(OtypeRefs::Name, serde_json::to_value(&table.name).unwrap());
        }
        result
    }
}

#[async_trait]
impl TableDefinition for Otype {
    type Id = Uuid;
    type References = OtypeRefs;
    type Loader = OtypeLoader;
    type Filter = OtypeFilter;
    type Sorting = OtypeSorting;

    fn new_loader(pool: &PgPool) -> Self::Loader {
        OtypeLoader(pool.clone())
    }

    fn id_column() -> Self::References {
        OtypeRefs::OtypeId
    }

    fn get_id(&self) -> Self::Id {
        self.otype_id
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
                .map(|x| Expr::value(x))
                .unwrap(),
            OtypeRefs::Name => serde_json::from_value::<String>(value)
                .map(|x| Expr::value(x))
                .unwrap(),
            OtypeRefs::Description => serde_json::from_value::<String>(value)
                .map(|x| Expr::value(x))
                .unwrap(),
            OtypeRefs::Definition => Expr::value(value),
            _ => unreachable!(),
        }
    }

    // fn parse_value(key: <Self::Table as TableDefinition>::References, value: Value) -> Result<SimpleExpr, serde_json::Error> {
    //     match key {
    //         OtypeRefs::OtypeId => serde_json::from_value::<Uuid>(value).map(|x| Expr::value(x)),
    //         OtypeRefs::Name => serde_json::from_value::<String>(value).map(|x| Expr::value(x)),
    //         OtypeRefs::Description => serde_json::from_value::<String>(value).map(|x| Expr::value(x)),
    //         OtypeRefs::Definition => Ok(Expr::value(value)),
    //         _ => unreachable!()
    //     }
    // }

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
                x.columns(vec![
                    OtypeRefs::OtypeId,
                    OtypeRefs::Name,
                    OtypeRefs::Description,
                    OtypeRefs::Definition,
                ])
                .from(OtypeRefs::Table);
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
        info!("Begin load otype");
        // 대상 id 전체 조회
        let result = sqlx::query_scalar_with::<_, Uuid, _>(&query, parames)
            .fetch_all(pool)
            .await?;
        // has_next, has_prev 계산
        // let prepared_has_next_prev = cursor.prepare_has_next_prev(&result, OtypeIdentity::OtypeId, (Public, OtypeIdentity::Table));
        // let (query, values) = prepared_has_next_prev.build_sqlx(PostgresQueryBuilder);
        // let row = sqlx::query_with(&query, values).fetch_one(&loader.0).await?;
        // let has_prev = row.get::<bool, _>(0);
        // let has_next = row.get::<bool, _>(1);
        // 결과물 리턴
        // let mut entities = loader.load_many(result.iter().cloned()).await?;
        // let mut connection = Connection::new(has_prev, has_next);
        let mut connection = Connection::new(false, false);
        // let result = result.into_iter()
        //     .map(async |otype_id| {
        //         Edge::new(otype_id.to_string(), otype_id)
        // }).collect();

        connection.edges.extend(
            join_all(result.into_iter().map(|otype_id| {
                FutureExt::map(loader.load_one(otype_id), |x| x.unwrap().unwrap())
                    .map(|x| Edge::new(sorting.encode_key(&x), x))
            }))
            .await,
        );

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

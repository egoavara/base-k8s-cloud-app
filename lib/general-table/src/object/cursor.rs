use async_graphql::connection::{CursorType, Edge};
use std::collections::HashMap;

use async_graphql::{InputObject, ObjectType};
use base64::Engine;
use sea_query::{Alias, ConditionalStatement, Expr, Iden, SelectStatement, SimpleExpr};
use tap::Tap;

use crate::config::CursorConfig;
use crate::traits::{FilterTable, SortingTable, TableDefinition};

#[derive(Debug, Default, Clone, InputObject)]
pub struct Cursor {
    pub after: Option<String>,
    pub first: Option<u32>,
    pub before: Option<String>,
    pub last: Option<u32>,
}

pub struct ValidatedCursor {
    pub after: HashMap<String, serde_json::Value>,
    pub before: HashMap<String, serde_json::Value>,
    pub limit: Option<u32>,
}

pub(crate) fn decode_key(
    dst: &mut HashMap<String, serde_json::Value>,
    key: &str,
    default_identity: String,
) -> Result<(), async_graphql::Error> {
    if key.starts_with("$:") {
        let key = key.strip_prefix("$:").unwrap();
        let key = base64::prelude::BASE64_STANDARD
            .decode(key.as_bytes())
            .unwrap();
        // TODO: error handling 추가
        let tmp = serde_json::from_slice::<HashMap<String, serde_json::Value>>(&key).unwrap();
        dst.extend(tmp);
    } else {
        dst.insert(default_identity, serde_json::Value::String(key.to_string()));
    }
    Ok(())
}

impl Cursor {
    pub fn validate<ST: SortingTable>(
        self,
        config: &CursorConfig,
        _sorting: &ST,
    ) -> Result<ValidatedCursor, async_graphql::Error> {
        let Cursor {
            after: raw_after,
            first,
            before: raw_before,
            last,
        } = self;

        let mut after = HashMap::new();
        let mut before = HashMap::new();
        // sorting.get_ordered_identities().iter().for_each(|(key, _)| {
        //     after.insert(key.to_string(), serde_json::Value::Null);
        //     before.insert(key.to_string(), serde_json::Value::Null);
        // });

        if let Some(raw_after) = raw_after {
            decode_key(
                &mut after,
                raw_after.as_ref(),
                ST::Table::id_column().to_string(),
            )?;
        }
        if let Some(raw_before) = raw_before {
            decode_key(
                &mut before,
                raw_before.as_ref(),
                ST::Table::id_column().to_string(),
            )?;
        }
        if first.is_some() && last.is_some() {
            return Err(async_graphql::Error::new(
                "first and last cannot be used together",
            ));
        }
        Ok(ValidatedCursor {
            after,
            before,
            limit: first.or(last).or(config.default_limit),
        })
    }
}

impl ValidatedCursor {
    pub fn apply<'a, T: ConditionalStatement, TB: TableDefinition, ST: SortingTable<Table = TB>>(
        &self,
        query: &'a mut T,
        sorting: &ST,
    ) -> &'a mut T {
        let values = sorting
            .safe_order_by_clause()
            .into_iter()
            .map(|(col, _)| {
                (
                    col.clone(),
                    self.before.get(&col.to_string()),
                    self.after.get(&col.to_string()),
                )
            })
            .collect::<Vec<_>>();
        let mut result = Vec::<SimpleExpr>::new();
        for i in 0..values.len() {
            let i_col = values[i].0.clone();
            // TODO : 에러 메시지 고도화
            // TODO : null 처리 추가
            let i_before = values[i]
                .1
                .map(|x| TB::decode_field(i_col.clone(), x.clone()));
            let i_after = values[i]
                .2
                .map(|x| TB::decode_field(i_col.clone(), x.clone()));
            // before
            if let Some(i_before) = i_before.clone() {
                let mut before_cond = Expr::col(i_col.clone()).lt(i_before.clone());
                for j in 0..i {
                    let j_col = values[j].0.clone();
                    let j_before = values[j]
                        .1
                        .map(|x| TB::decode_field(i_col.clone(), x.clone()));
                    if let Some(j_before) = j_before {
                        before_cond = before_cond.and(Expr::col(j_col).eq(j_before.clone()));
                    }
                }
                result.push(before_cond);
            }
            // after
            if let Some(i_after) = i_after {
                let mut after_cond = Expr::col(i_col.clone()).gt(i_after.clone());
                for j in 0..i {
                    let j_col = values[j].0.clone();
                    let j_after = values[j]
                        .2
                        .map(|x| TB::decode_field(j_col.clone(), x.clone()));
                    if let Some(j_after) = j_after {
                        after_cond = after_cond.and(Expr::col(j_col).eq(j_after.clone()));
                    }
                }
                result.push(after_cond);
            }
        }
        if let Some(cond) = result.into_iter().reduce(|a, b| a.or(b)) {
            query.and_where(cond);
        }
        query
    }

    fn cursor_after<TB: TableDefinition>(
        &self,
        data: &TB,
        cols: &Vec<TB::References>,
    ) -> SimpleExpr {
        cols.iter()
            .enumerate()
            .map(|(i, col)| {
                let gt = Expr::col(col.clone()).gt(TB::decode_field(
                    col.clone(),
                    data.encode_field(col.clone()),
                ));
                cols.iter()
                    .take(i)
                    .map(|eq_col| {
                        Expr::col(eq_col.clone()).eq(TB::decode_field(
                            eq_col.clone(),
                            data.encode_field(eq_col.clone()),
                        ))
                    })
                    .fold(gt, |acc, curr| acc.and(curr))
            })
            .reduce(|a, b| a.or(b))
            .unwrap_or_else(|| Expr::value(false))
    }

    fn cursor_before<TB: TableDefinition>(
        &self,
        data: &TB,
        cols: &Vec<TB::References>,
    ) -> SimpleExpr {
        cols.iter()
            .enumerate()
            .map(|(i, col)| {
                let lt = Expr::col(col.clone()).lt(TB::decode_field(
                    col.clone(),
                    data.encode_field(col.clone()),
                ));
                cols.iter()
                    .take(i)
                    .map(|eq_col| {
                        Expr::col(eq_col.clone()).eq(TB::decode_field(
                            eq_col.clone(),
                            data.encode_field(eq_col.clone()),
                        ))
                    })
                    .fold(lt, |acc, curr| acc.and(curr))
            })
            .reduce(|a, b| a.or(b))
            .unwrap_or_else(|| Expr::value(false))
    }

    pub fn prepare_cursor_result<CS, TB, EM, FT, ST>(
        &self,
        result: &Vec<Edge<CS, TB, EM>>,
        ft: &FT,
        st: &ST,
    ) -> SelectStatement
    where
        CS: CursorType + Send + Sync,
        TB: TableDefinition,
        EM: ObjectType,
        ST: SortingTable<Table = TB>,
        FT: FilterTable<Table = TB>,
    {
        let cols = st
            .safe_order_by_clause()
            .into_iter()
            .map(|(col, _)| col)
            .collect::<Vec<_>>();
        if cols.is_empty() {
            return SelectStatement::new()
                .expr_as(Expr::value(false), Alias::new("has_prev"))
                .expr_as(Expr::value(false), Alias::new("has_next"))
                .to_owned();
        }
        // has_prev
        let has_prev = match result.first() {
            Some(first) => SelectStatement::new()
                .column(TB::id_column())
                .from(TB::table())
                .and_where(self.cursor_before(&first.node, &cols))
                .tap_deref_mut(|x| {
                    ft.apply(x);
                })
                .to_owned(),
            None => SelectStatement::new().expr(Expr::value(false)).to_owned(),
        };
        // has_next
        let has_next = match result.last() {
            Some(last) => SelectStatement::new()
                .column(TB::id_column())
                .from(TB::table())
                .and_where(self.cursor_after(&last.node, &cols))
                .tap_deref_mut(|x| {
                    ft.apply(x);
                })
                .to_owned(),
            None => SelectStatement::new().expr(Expr::value(false)).to_owned(),
        };
        SelectStatement::new()
            .expr_as(Expr::exists(has_prev), Alias::new("has_prev"))
            .expr_as(Expr::exists(has_next), Alias::new("has_next"))
            .to_owned()
    }
}

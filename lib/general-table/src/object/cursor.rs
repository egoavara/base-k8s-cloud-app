use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use async_graphql::InputObject;
use base64::Engine;
use itertools::Itertools;
use sea_query::{ConditionalStatement, Expr, Iden, SimpleExpr};
use serde::de::DeserializeOwned;
use sqlx::Database;

use crate::config::CursorConfig;
use crate::traits::{SortingTable, TableDefinition};

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
        dst.extend(tmp.into_iter());
    } else {
        dst.insert(
            default_identity.into(),
            serde_json::Value::String(key.to_string()),
        );
    }
    Ok(())
}

impl Cursor {
    pub fn validate<ST: SortingTable>(
        self,
        config: &CursorConfig,
        sorting: &ST,
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
            limit: first.or_else(|| last).or(config.default_limit),
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
            .order_by_clause()
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
                    // let j_before = sorting.decode_key(i_col.clone(), values[j].1.clone()).unwrap();
                    let j_before = values[j]
                        .1
                        .clone()
                        .map(|x| TB::decode_field(i_col.clone(), x.clone()));
                    if let Some(j_before) = j_before {
                        before_cond = before_cond.and(Expr::col(j_col).eq(j_before.clone()));
                    }
                }
                result.push(before_cond);
            }
            // after
            if let Some(i_after) = i_after.clone() {
                let mut after_cond = Expr::col(i_col.clone()).gt(i_after.clone());
                for j in 0..i {
                    let j_col = values[j].0.clone();
                    let j_after = values[j]
                        .2
                        .clone()
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
    // pub fn condition<'qb, 'args: 'qb, ST: SortingTable>(&self, builder: &mut QueryBuilder<'args, Postgres>, config: &CursorConfig, sorting: &ST) {
    //     let values = sorting.get_ordered_identities().into_iter()
    //         .map(|(col, _)| (col, self.before.get(col).unwrap(), self.after.get(col).unwrap()))
    //         .filter(|(_, before, after)| !(before.is_null() && after.is_null()))
    //         .collect::<Vec<_>>();
    //     if values.is_empty() { return; }
    //
    //     builder.push(" and ((");
    //
    //     let mut binder = builder.separated(") or (".to_string());
    //     builder.push(")) ");
    // }
    // pub fn order_by<'args, ST: SortingTable>(&'args self, builder: &mut QueryBuilder<'args, Postgres>, sorting: &ST) {
    //     builder.push(" order by ");
    //     let mut order_by_clause = builder.separated(", ");
    //     for (ident, direction) in sorting.get_ordered_identities() {
    //         match direction {
    //             SortDirection::Ascending => {
    //                 order_by_clause.push(format!("{} asc", ident));
    //             }
    //             SortDirection::Descending => {
    //                 order_by_clause.push(format!("{} desc", ident));
    //             }
    //         }
    //     }
    // }
}

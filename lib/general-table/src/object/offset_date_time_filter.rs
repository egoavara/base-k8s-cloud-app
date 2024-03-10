use crate::scalar::OffsetDateTimeScalar;
use crate::traits::FilterField;
use async_graphql::InputObject;
use sea_query::{Expr, IntoColumnRef, SimpleExpr};
use sqlx::query_builder::Separated;
use sqlx::{Database, Postgres};
use std::fmt::Display;

#[derive(Debug, Clone, Default, InputObject)]
pub struct OffsetDateTimeFilter {
    pub gt: Option<OffsetDateTimeScalar>,
    pub gte: Option<OffsetDateTimeScalar>,
    pub lt: Option<OffsetDateTimeScalar>,
    pub lte: Option<OffsetDateTimeScalar>,
    // pub timezone: Option<String>,
}

impl FilterField for OffsetDateTimeFilter {
    fn to_expr(&self, column: impl IntoColumnRef + Clone) -> Option<SimpleExpr> {
        let mut result: Vec<SimpleExpr> = Vec::new();
        if let Some(gt) = &self.gt {
            result.push(Expr::col(column.clone()).gt(gt.to_string()));
        }
        if let Some(gte) = &self.gte {
            result.push(Expr::col(column.clone()).gte(gte.to_string()));
        }
        if let Some(lt) = &self.lt {
            result.push(Expr::col(column.clone()).lt(lt.to_string()));
        }
        if let Some(lte) = &self.lte {
            result.push(Expr::col(column.clone()).lte(lte.to_string()));
        }
        result.into_iter().reduce(|a, b| a.and(b))
    }
}

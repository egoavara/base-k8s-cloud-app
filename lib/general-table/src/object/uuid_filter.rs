use async_graphql::InputObject;
use sea_query::{Expr, IntoColumnRef, SimpleExpr};
use sqlx::types::Uuid;

use crate::traits::FilterField;

#[derive(Debug, Clone, Default, InputObject)]
pub struct UuidFilter {
    pub eq: Option<Uuid>,
    pub r#in: Option<Vec<Uuid>>,
}

impl FilterField for UuidFilter {
    fn to_expr(&self, column: impl IntoColumnRef + Clone) -> Option<SimpleExpr> {
        let mut result: Vec<SimpleExpr> = Vec::new();
        if let Some(eq) = &self.eq {
            result.push(Expr::col(column.clone()).eq(Expr::value(*eq)));
        }
        if let Some(r#in) = &self.r#in {
            result
                .push(Expr::col(column.clone()).is_in(r#in.iter().map(|x| x.to_string()).clone()));
        }
        result.into_iter().reduce(|a, b| a.and(b))
    }
}

use async_graphql::InputObject;
use sea_query::{Expr, IntoColumnRef, SimpleExpr};

use crate::traits::{escape_like_pg, FilterField};

#[derive(Debug, Clone, Default, InputObject)]
pub struct NameFilter {
    pub eq: Option<String>,
    pub ne: Option<String>,

    pub r#in: Option<Vec<String>>,
    pub nin: Option<Vec<String>>,

    pub contains: Option<String>,
    pub ncontains: Option<String>,
    pub starts_with: Option<String>,
    pub nstarts_with: Option<String>,

    pub like: Option<String>,
    pub nlike: Option<String>,

    pub query: Option<String>,
}

impl FilterField for NameFilter {
    fn to_expr(&self, column: impl IntoColumnRef + Clone) -> Option<SimpleExpr> {
        let mut result: Vec<SimpleExpr> = Vec::new();
        if let Some(eq) = &self.eq {
            result.push(Expr::col(column.clone()).eq(eq));
        }
        if let Some(ne) = &self.ne {
            result.push(Expr::col(column.clone()).ne(ne));
        }
        if let Some(r#in) = &self.r#in {
            result.push(Expr::col(column.clone()).is_in(r#in.clone()));
        }
        if let Some(nin) = &self.nin {
            result.push(Expr::col(column.clone()).is_not_in(nin.clone()));
        }
        if let Some(contains) = &self.contains {
            result.push(Expr::col(column.clone()).like("%".to_owned() + contains + "%"));
        }
        if let Some(ncontains) = &self.ncontains {
            result.push(
                Expr::col(column.clone())
                    .like("%".to_owned() + ncontains + "%")
                    .not(),
            );
        }
        if let Some(starts_with) = &self.starts_with {
            result.push(Expr::col(column.clone()).like(escape_like_pg(starts_with) + "%"));
        }
        if let Some(nstarts_with) = &self.nstarts_with {
            result.push(
                Expr::col(column.clone())
                    .like(escape_like_pg(nstarts_with) + "%")
                    .not(),
            );
        }
        if let Some(like) = &self.like {
            result.push(Expr::col(column.clone()).like(like));
        }
        if let Some(nlike) = &self.nlike {
            result.push(Expr::col(column.clone()).like(nlike).not());
        }
        result.into_iter().reduce(|x, y| x.and(y))
    }
}

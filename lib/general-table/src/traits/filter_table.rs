use std::convert::identity;

use sea_query::{ConditionalStatement, SimpleExpr};

use crate::traits::TableDefinition;

pub trait FilterTable {
    type Table: TableDefinition;

    fn where_clause(&self) -> Vec<Option<SimpleExpr>>;

    fn apply<'a, T: ConditionalStatement>(&self, query: &'a mut T) -> &'a mut T {
        let r#where = self.where_clause();
        if r#where.is_empty() {
            return query;
        }
        r#where
            .into_iter()
            .filter_map(identity)
            .fold(query, |query, expr| query.and_where(expr))
    }
}

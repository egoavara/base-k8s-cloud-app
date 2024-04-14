use std::collections::HashMap;
use std::future::Future;

use async_graphql::connection::Connection;
use async_graphql::OutputType;
use sea_query::{ColumnRef, NullOrdering, SelectStatement, TableRef};

use crate::page::Page;
use crate::types::{ConnectionError, Context};
use crate::{CursorWrap, Field, PageByCursor, PageByCursorVariant, Table, TableFilter, TableSorter};

//
impl<'a, T: Table + OutputType> Context<'a, T> {
    // pub fn connection_direct<T, F>(
    //     &self,
    //     page: Page<T::Cursor>,
    //     filter: T::Filter,
    //     sorter: T::Sorter,
    //     f: F,
    // ) -> Connection<CursorWrap<T::Cursor>, T>
    // where
    //     T: Table + ObjectType,
    //     F: FnOnce(String, Values) -> Vec<T>,
    // {
    //     let (query, values) = T::query_many_as(Some(self), "hello", cursor, filter, sorter)
    //         .build(sea_query::PostgresQueryBuilder);
    //     let data = f(query, values);
    //     let mut result = Connection::new(false, false);
    //     result.edges.extend(data.into_iter().map(|node| {
    //         async_graphql::connection::Edge::new(CursorWrap::new(node.to_cursor()), node)
    //     }));
    //     result
    // }
    pub async fn connection<CR0, C0, FR0, F0, FR1, F1>(&mut self, page: Option<Page<T::Cursor>>, filter: T::Filter, sorter: T::Sorter, c0: C0, f0: F0, f1: F1) -> Result<Connection<CursorWrap<T::Cursor>, T>, ConnectionError>
    where
        CR0: Future<Output = Option<Vec<T::Id>>>,
        C0: FnOnce(&T::Cursor) -> CR0,
        FR0: Future<Output = Vec<T::Id>>,
        F0: FnOnce(SelectStatement) -> FR0,
        FR1: Future<Output = HashMap<T::Id, T>>,
        F1: FnOnce(Vec<T::Id>) -> FR1,
    {
        let page = page.unwrap_or_default();
        match page {
            Page::Cursor(cursor) => self.connection_by_cursor(cursor, filter, sorter, c0, f0, f1).await,
        }
    }

    pub async fn connection_by_cursor<CR0, C0, FR0, F0, FR1, F1>(&mut self, page: PageByCursor<T::Cursor>, filter: T::Filter, sorter: T::Sorter, c0: C0, f0: F0, f1: F1) -> Result<Connection<CursorWrap<T::Cursor>, T>, ConnectionError>
    where
        CR0: Future<Output = Option<Vec<T::Id>>>,
        C0: FnOnce(&T::Cursor) -> CR0,
        FR0: Future<Output = Vec<T::Id>>,
        F0: FnOnce(SelectStatement) -> FR0,
        FR1: Future<Output = HashMap<T::Id, T>>,
        F1: FnOnce(Vec<T::Id>) -> FR1,
    {
        let page_variant = page.into_variant()?;

        let result_query = self.prepare_query(self.prepare_all_column_refs(), filter, sorter, page_variant);

        let ids = f0(result_query).await;
        let mut data = f1(ids.clone()).await;
        let mut result = Connection::new(false, false);
        for id in ids {
            let node = data.remove(&id).unwrap();
            result.edges.push(async_graphql::connection::Edge::new(CursorWrap::new(node.to_cursor()), node));
        }
        Ok(result)
    }

    fn prepare_id_column_refs(&self) -> Vec<ColumnRef> {
        T::id_fields().into_iter().map(|x| self.column_ref(x.column_ident())).collect::<Vec<ColumnRef>>()
    }
    fn prepare_all_column_refs(&self) -> Vec<ColumnRef> {
        T::fields().into_iter().map(|x| self.column_ref(x.column_ident())).collect::<Vec<ColumnRef>>()
    }
    fn prepare_query(&mut self, id_column_refs: Vec<ColumnRef>, filter: <T as Table>::Filter, sorter: <T as Table>::Sorter, page_variant: PageByCursorVariant<<T as Table>::Cursor>) -> SelectStatement {
        let table_ref = self.table_ref(T::table());

        let mut query = SelectStatement::new();
        query.columns(id_column_refs);
        query.from(table_ref);
        query.cond_where(filter.to_condition(self));
        match page_variant {
            PageByCursorVariant::After { after: None, limit } | PageByCursorVariant::Before { before: None, limit } => {
                if let Some(limit) = limit {
                    query.limit(limit);
                }
            }
            PageByCursorVariant::After { after: Some(after), limit } => {
                query.cond_where(sorter.to_after_condition(after.0, self));
                if let Some(limit) = limit {
                    query.limit(limit);
                }
            }
            PageByCursorVariant::Before { before: Some(before), limit } => {
                todo!()
            }
            PageByCursorVariant::Between { .. } => {
                todo!()
            }
            PageByCursorVariant::BetweenRev { .. } => {
                todo!()
            }
        }
        for (expr, order, nulls) in sorter.to_order(self) {
            query.order_by_expr_with_nulls(expr, order, nulls.unwrap_or(NullOrdering::First));
        }
        query
    }
}

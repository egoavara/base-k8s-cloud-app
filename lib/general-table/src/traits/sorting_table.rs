use std::collections::HashMap;

use base64::Engine;
use sea_query::{Iden, Order, OrderedStatement};
use serde_json::Value;

use crate::scalar::SortDirection;
use crate::traits::TableDefinition;

pub trait SortingTable {
    type Table: TableDefinition;

    fn order_by_clause(&self)
        -> Vec<(<Self::Table as TableDefinition>::References, SortDirection)>;
    fn apply<'a, T: OrderedStatement>(&self, query: &'a mut T) -> &'a mut T {
        let order_by = self.order_by_clause();
        if order_by.is_empty() {
            return query.order_by(Self::Table::id_column(), Order::Asc);
        }
        order_by
            .into_iter()
            .fold(query, |query, (column, direction)| {
                query.order_by(column, direction.into())
            })
    }

    fn key_candidate(
        &self,
        table: &Self::Table,
    ) -> HashMap<<Self::Table as TableDefinition>::References, serde_json::Value> {
        self.order_by_clause()
            .iter()
            .map(|(k, _)| (k.clone(), table.encode_field(k.clone())))
            .collect()
    }
    fn encode_key(&self, table: &Self::Table) -> String {
        let key_candidate = self
            .key_candidate(table)
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<HashMap<String, Value>>();
        if key_candidate.is_empty() {
            panic!("key_candidate is empty")
        }
        match key_candidate.get(&Self::Table::id_column().to_string()) {
            Some(data) if key_candidate.len() == 1 => data.to_string(),
            _ => {
                let json_key_candidate = serde_json::to_vec(&key_candidate).unwrap();
                let base64_key_candidate =
                    base64::prelude::BASE64_STANDARD.encode(&json_key_candidate);
                format!("$:{}", base64_key_candidate)
            }
        }
    }
}

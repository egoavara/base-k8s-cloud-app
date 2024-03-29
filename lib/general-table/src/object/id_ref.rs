use crate::traits::TableDefinition;
use async_graphql::{QueryPathNode, QueryPathSegment};
use itertools::Itertools;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct IdRef<F: TableDefinition, T: TableDefinition> {
    pub id: F::Id,
    pub filter: Option<T::Filter>,
    pub group: Option<String>,
    _phantom: std::marker::PhantomData<(F, T)>,
}

impl<F: TableDefinition, T: TableDefinition> IdRef<F, T> {
    pub fn new(group: Option<String>, id: F::Id, filter: Option<T::Filter>) -> Self {
        Self {
            id,
            filter,
            group,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn filter(id: F::Id, filter: Option<T::Filter>) -> Self {
        Self {
            id,
            filter,
            group: None,
            _phantom: std::marker::PhantomData,
        }
    }

    // pub fn group_by<'a>(path: impl Into<Option<QueryPathNode<'a>>>, id: T::Id) -> Self {
    //     let path = path.into();
    //     if let Some(path) = &path {
    //         return Self {
    //             id,
    //             filter: None,
    //             group: Self::pick_path_names(&path).into(),
    //             _phantom: std::marker::PhantomData,
    //         };
    //     }
    //     Self::new(id)
    // }
    pub fn group_by_filter<'a>(
        path: impl Into<Option<QueryPathNode<'a>>>,
        id: F::Id,
        filter: Option<T::Filter>,
    ) -> Self {
        let path = path.into();
        if let Some(_path) = &path {
            return Self {
                id,
                filter,
                group: Some("same".to_string()),
                // group: Self::pick_path_names(path).into(),
                _phantom: std::marker::PhantomData,
            };
        }
        Self::filter(id, filter)
    }
    fn pick_path_names<'a>(path: &'a QueryPathNode<'a>) -> String {
        let mut result = Vec::new();
        let mut current = Some(path);
        while let Some(x) = current {
            if let QueryPathSegment::Name(name) = x.segment {
                result.push(name);
            }
            current = x.parent;
        }
        result.iter().rev().join(".")
    }
}

impl<F: TableDefinition, T: TableDefinition> PartialEq for IdRef<F, T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.group == other.group
    }
}

impl<F: TableDefinition, T: TableDefinition> Eq for IdRef<F, T> {}

impl<F: TableDefinition, T: TableDefinition> Hash for IdRef<F, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.group.hash(state);
    }
}

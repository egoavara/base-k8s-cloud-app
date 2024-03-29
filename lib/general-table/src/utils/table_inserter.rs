use std::borrow::Cow;

use std::sync::Arc;

use async_graphql::async_trait::async_trait;
use async_graphql::parser::types::Field;
use async_graphql::registry::{Deprecation, MetaField, MetaType, MetaTypeId, Registry};
use async_graphql::{
    CacheControl, ContainerType, Context, ContextSelectionSet, Enum, InputType, OutputType,
    Positioned, ServerError, ServerResult,
};
use async_graphql_value::ConstValue;
use tokio::sync::RwLock;

use crate::traits::{InsertingError, InsertingTable};

#[derive()]
pub struct TableInserter<T: InsertingTable> {
    data: T,
    inner: Arc<RwLock<TableInserterInner<T::Returning, T::Failure>>>,
}

struct TableInserterInner<R, F> {
    result: TableInsertResult,
    returning: Option<R>,
    failure: Option<F>,
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Enum)]
pub enum TableInsertResult {
    Success,
    Failure,
    Ignored,
}

impl<T: InsertingTable> TableInserter<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            inner: Arc::new(RwLock::new(TableInserterInner {
                result: TableInsertResult::Ignored,
                failure: None,
                returning: None,
            })),
        }
    }
}

#[async_trait]
impl<T: InsertingTable> OutputType for TableInserter<T> {
    fn type_name() -> Cow<'static, str> {
        Cow::Owned(format!(
            "Inserting{}",
            <T as InsertingTable>::Returning::type_name()
        ))
    }

    fn qualified_type_name() -> String {
        format!(
            "Inserting{}",
            <T as InsertingTable>::Returning::qualified_type_name()
        )
    }

    fn introspection_type_name(&self) -> Cow<'static, str> {
        Self::type_name()
    }

    fn create_type_info(registry: &mut Registry) -> String {
        registry.create_output_type::<Self, _>(MetaTypeId::Object, |registry| MetaType::Object {
            name: Self::type_name().to_string(),
            description: None,
            fields: {
                let mut fields = async_graphql::indexmap::IndexMap::new();
                fields.insert(
                    ToOwned::to_owned("result"),
                    MetaField {
                        name: ToOwned::to_owned("result"),
                        description: None,
                        args: Default::default(),
                        ty: <TableInsertResult as OutputType>::create_type_info(registry),
                        deprecation: Deprecation::NoDeprecated,
                        cache_control: CacheControl {
                            public: true,
                            max_age: 0i32,
                        },
                        external: false,
                        provides: None,
                        requires: None,
                        shareable: false,
                        inaccessible: false,
                        tags: Vec::new(),
                        override_from: None,
                        visible: None,
                        compute_complexity: None,
                        directive_invocations: Vec::new(),
                    },
                );
                fields.insert(
                    ToOwned::to_owned("returning"),
                    MetaField {
                        name: ToOwned::to_owned("returning"),
                        description: None,
                        args: Default::default(),
                        ty: <Option<T::Returning> as OutputType>::create_type_info(registry),
                        deprecation: Deprecation::NoDeprecated,
                        cache_control: CacheControl {
                            public: true,
                            max_age: 0i32,
                        },
                        external: false,
                        provides: None,
                        requires: None,
                        shareable: false,
                        inaccessible: false,
                        tags: Vec::new(),
                        override_from: None,
                        visible: None,
                        compute_complexity: None,
                        directive_invocations: Vec::new(),
                    },
                );
                fields.insert(
                    ToOwned::to_owned("failure"),
                    MetaField {
                        name: ToOwned::to_owned("failure"),
                        description: None,
                        args: Default::default(),
                        ty: <Option<T::Failure> as OutputType>::create_type_info(registry),
                        deprecation: Deprecation::NoDeprecated,
                        cache_control: CacheControl {
                            public: true,
                            max_age: 0i32,
                        },
                        external: false,
                        provides: None,
                        requires: None,
                        shareable: false,
                        inaccessible: false,
                        tags: Vec::new(),
                        override_from: None,
                        visible: None,
                        compute_complexity: None,
                        directive_invocations: Vec::new(),
                    },
                );
                fields
            },
            cache_control: CacheControl {
                public: true,
                max_age: 0i32,
            },
            extends: false,
            shareable: false,
            resolvable: true,
            inaccessible: false,
            interface_object: false,
            tags: Vec::new(),
            keys: None,
            visible: None,
            is_subscription: false,
            rust_typename: Some(::std::any::type_name::<Self>()),
            directive_invocations: Vec::new(),
        })
    }

    async fn resolve(
        &self,
        ctx: &ContextSelectionSet<'_>,
        _field: &Positioned<Field>,
    ) -> ServerResult<ConstValue> {
        let nctx = ctx
            .query_env
            .create_context(ctx.schema_env, ctx.path_node, ctx.item);
        async_graphql::resolver_utils::resolve_container(&nctx, self).await
    }
}

#[async_trait]
impl<T: InsertingTable> ContainerType for TableInserter<T> {
    async fn resolve_field(&self, ctx: &Context<'_>) -> ServerResult<Option<ConstValue>> {
        let read = {
            let read = self.inner.read().await;
            if read.result != TableInsertResult::Ignored {
                Ok(read)
            } else {
                drop(read);
                let mut write = self.inner.write().await;
                if write.result == TableInsertResult::Ignored {
                    let result = T::insert(&self.data, ctx).await;
                    let err = match result {
                        Ok(returning) => {
                            write.result = TableInsertResult::Success;
                            write.returning.replace(returning);
                            write.failure.take();
                            None
                        }
                        Err(InsertingError::Throw(failure)) => {
                            write.result = TableInsertResult::Failure;
                            write.returning.take();
                            write.failure.replace(failure);
                            None
                        }
                        Err(InsertingError::Unexpected(e)) => Some(e),
                    };
                    if let Some(err) = err {
                        Err(err)
                    } else {
                        drop(write);
                        Ok(self.inner.read().await)
                    }
                } else {
                    drop(write);
                    Ok(self.inner.read().await)
                }
            }
        }
        .map_err(|e| ServerError::new(e.message.to_string(), None))?;

        if ctx.item.node.name.node == "returning" {
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return OutputType::resolve(&read.returning, &ctx_obj, ctx.item)
                .await
                .map(Some);
        }
        if ctx.item.node.name.node == "failure" {
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return OutputType::resolve(&read.failure, &ctx_obj, ctx.item)
                .await
                .map(Some);
        }
        if ctx.item.node.name.node == "result" {
            let ctx_obj = ctx.with_selection_set(&ctx.item.node.selection_set);
            return OutputType::resolve(&read.result, &ctx_obj, ctx.item)
                .await
                .map(Some);
        }
        Ok(None)
    }
}

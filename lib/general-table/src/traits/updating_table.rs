use async_graphql::registry::{MetaInputValue, Registry};

pub trait UpdatingTable {
    fn metadata<H: FnOnce(String) -> MetaInputValue>(
        registry: &mut Registry,
        handler: H,
    ) -> Option<MetaInputValue>;
}

pub struct NotUpdatable;

impl UpdatingTable for NotUpdatable {
    fn metadata<H: FnOnce(String) -> MetaInputValue>(
        _registry: &mut Registry,
        _handler: H,
    ) -> Option<MetaInputValue> {
        None
    }
}

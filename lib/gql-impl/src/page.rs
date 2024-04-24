use std::borrow::Cow;

use crate::types::PageCursorVariantError;
use crate::{Cursor, CursorWrap};
use async_graphql::indexmap::IndexMap;
use async_graphql::registry::{MetaInputValue, MetaType, MetaTypeId, Registry};
use async_graphql::{InputObjectType, InputType, InputValueResult};

#[derive(Debug)]
pub enum Page<T: Cursor> {
    Cursor(PageByCursor<T>),
}

#[derive(Default, Debug)]
pub struct PageByCursor<T: Cursor> {
    pub after: Option<CursorWrap<T>>,
    pub before: Option<CursorWrap<T>>,
    pub first: Option<u64>,
    pub last: Option<u64>,
}

#[derive(Clone)]
pub enum PageByCursorVariant<T: Cursor> {
    After { after: Option<CursorWrap<T>>, limit: Option<u64> },
    Before { before: Option<CursorWrap<T>>, limit: Option<u64> },
    Between { after: CursorWrap<T>, before: CursorWrap<T>, limit: Option<u64> },
    BetweenRev { after: CursorWrap<T>, before: CursorWrap<T>, limit: Option<u64> },
}

impl<T: Cursor> Default for Page<T> {
    fn default() -> Self {
        Page::Cursor(PageByCursor {
            after: None,
            before: None,
            first: None,
            last: None,
        })
    }
}

impl<T: Cursor> From<PageByCursor<T>> for Page<T> {
    fn from(value: PageByCursor<T>) -> Self {
        Page::Cursor(value)
    }
}

impl<T: Cursor> PageByCursor<T> {
    pub fn into_variant(self) -> Result<PageByCursorVariant<T>, PageCursorVariantError> {
        match (self.after, self.before, self.first, self.last) {
            // error cases
            (_, _, Some(_), Some(_)) => Err(PageCursorVariantError::BothFirstAndLast),
            (None, Some(_), Some(_), _) => Err(PageCursorVariantError::BeforeWithFirst),
            (Some(_), None, None, Some(_)) => Err(PageCursorVariantError::AfterWithLast),
            // normal cases
            (after, None, first, None) => Ok(PageByCursorVariant::After { after, limit: first }),
            (None, before, None, last) => Ok(PageByCursorVariant::Before { before, limit: last }),
            (Some(after), Some(before), first, None) => Ok(PageByCursorVariant::Between { after, before, limit: first }),
            (Some(after), Some(before), None, Some(last)) => Ok(PageByCursorVariant::BetweenRev { after, before, limit: Some(last) }),
        }
    }
}

impl<T: Cursor> InputType for PageByCursor<T> {
    type RawValueType = Self;
    fn type_name() -> Cow<'static, str> {
        Cow::Borrowed(T::type_name())
    }
    fn create_type_info(registry: &mut Registry) -> String {
        registry.create_input_type::<Self, _>(MetaTypeId::InputObject, |registry| MetaType::InputObject {
            name: Cow::into_owned(Cow::Borrowed(T::type_name())),
            description: None,
            input_fields: {
                let mut fields = IndexMap::new();
                fields.insert(
                    ToOwned::to_owned("after"),
                    MetaInputValue {
                        name: ToString::to_string("after"),
                        description: None,
                        ty: <Option<CursorWrap<T>> as InputType>::create_type_info(registry),
                        default_value: None,
                        visible: None,
                        inaccessible: false,
                        tags: Vec::new(),
                        is_secret: false,
                    },
                );
                fields.insert(
                    ToOwned::to_owned("before"),
                    MetaInputValue {
                        name: ToString::to_string("before"),
                        description: None,
                        ty: <Option<CursorWrap<T>> as InputType>::create_type_info(registry),
                        default_value: None,
                        visible: None,
                        inaccessible: false,
                        tags: Vec::new(),
                        is_secret: false,
                    },
                );
                fields.insert(
                    ToOwned::to_owned("first"),
                    MetaInputValue {
                        name: ToString::to_string("first"),
                        description: None,
                        ty: <Option<u64> as InputType>::create_type_info(registry),
                        default_value: None,
                        visible: None,
                        inaccessible: false,
                        tags: Vec::new(),
                        is_secret: false,
                    },
                );
                fields.insert(
                    ToOwned::to_owned("last"),
                    MetaInputValue {
                        name: ToString::to_string("last"),
                        description: None,
                        ty: <Option<u64> as InputType>::create_type_info(registry),
                        default_value: None,
                        visible: None,
                        inaccessible: false,
                        tags: Vec::new(),
                        is_secret: false,
                    },
                );
                fields
            },
            visible: None,
            inaccessible: false,
            tags: Vec::new(),
            rust_typename: Some(std::any::type_name::<Self>()),
            oneof: false,
        })
    }
    fn parse(value: Option<async_graphql::Value>) -> InputValueResult<Self> {
        if let Some(async_graphql::Value::Object(obj)) = value {
            #[allow(non_snake_case, unused_mut)]
            let mut after: Option<CursorWrap<T>> = InputType::parse(obj.get("after").cloned()).map_err(async_graphql::InputValueError::propagate)?;
            #[allow(non_snake_case, unused_mut)]
            let mut before: Option<CursorWrap<T>> = InputType::parse(obj.get("before").cloned()).map_err(async_graphql::InputValueError::propagate)?;
            #[allow(non_snake_case, unused_mut)]
            let mut first: Option<u64> = InputType::parse(obj.get("first").cloned()).map_err(async_graphql::InputValueError::propagate)?;
            #[allow(non_snake_case, unused_mut)]
            let mut last: Option<u64> = InputType::parse(obj.get("last").cloned()).map_err(async_graphql::InputValueError::propagate)?;
            let obj = Self { after, before, first, last };
            Ok(obj)
        } else {
            Err(async_graphql::InputValueError::expected_type(value.unwrap_or_default()))
        }
    }
    fn to_value(&self) -> async_graphql::Value {
        let mut map = IndexMap::new();
        map.insert(async_graphql::Name::new("after"), InputType::to_value(&self.after));
        map.insert(async_graphql::Name::new("before"), InputType::to_value(&self.before));
        map.insert(async_graphql::Name::new("first"), InputType::to_value(&self.first));
        map.insert(async_graphql::Name::new("last"), InputType::to_value(&self.last));
        async_graphql::Value::Object(map)
    }
    fn federation_fields() -> Option<String> {
        let mut res = Vec::new();
        if let Some(fields) = <Option<CursorWrap<T>> as InputType>::federation_fields() {
            res.push({
                let res = format!("{} {}", "after", fields);
                res
            });
        } else {
            res.push(ToString::to_string("after"));
        }
        if let Some(fields) = <Option<CursorWrap<T>> as InputType>::federation_fields() {
            res.push({
                let res = format!("{} {}", "before", fields);
                res
            });
        } else {
            res.push(ToString::to_string("before"));
        }
        if let Some(fields) = <Option<u64> as InputType>::federation_fields() {
            res.push({
                let res = format!("{} {}", "first", fields);
                res
            });
        } else {
            res.push(ToString::to_string("first"));
        }
        if let Some(fields) = <Option<u64> as InputType>::federation_fields() {
            res.push({
                let res = format!("{} {}", "last", fields);
                res
            });
        } else {
            res.push(ToString::to_string("last"));
        }
        Some({
            let res = format!("{{ {} }}", res.join(" "));
            res
        })
    }
    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }
}
impl<T: Cursor> InputObjectType for PageByCursor<T> {}

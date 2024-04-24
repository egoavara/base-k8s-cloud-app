use async_graphql::InputType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl<T: InputType> InputType for Range<T> {
    type RawValueType = Self;
    fn type_name() -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Owned("Range".to_owned() + T::type_name().as_ref())
    }
    fn create_type_info(registry: &mut async_graphql::registry::Registry) -> ::std::string::String {
        registry.create_input_type::<Self, _>(async_graphql::registry::MetaTypeId::InputObject, |registry| async_graphql::registry::MetaType::InputObject {
            name: Self::type_name().to_string(),
            description: None,
            input_fields: {
                let mut fields = async_graphql::indexmap::IndexMap::new();
                fields.insert(
                    ::std::borrow::ToOwned::to_owned("min"),
                    async_graphql::registry::MetaInputValue {
                        name: ToString::to_string("min"),
                        description: None,
                        ty: T::create_type_info(registry),
                        default_value: None,
                        visible: None,
                        inaccessible: false,
                        tags: Vec::new(),
                        is_secret: false,
                    },
                );
                fields.insert(
                    ::std::borrow::ToOwned::to_owned("max"),
                    async_graphql::registry::MetaInputValue {
                        name: ToString::to_string("max"),
                        description: None,
                        ty: T::create_type_info(registry),
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
            tags: Vec::new(),
            inaccessible: false,
            rust_typename: Some(::std::any::type_name::<Self>()),
            oneof: false,
        })
    }
    fn parse(value: Option<async_graphql::Value>) -> async_graphql::InputValueResult<Self> {
        if let Some(async_graphql::Value::Object(obj)) = value {
            let mut min: T = async_graphql::InputType::parse(obj.get("min").cloned()).map_err(async_graphql::InputValueError::propagate)?;

            let mut max: T = async_graphql::InputType::parse(obj.get("max").cloned()).map_err(async_graphql::InputValueError::propagate)?;
            let obj = Self { min, max };
            Ok(obj)
        } else {
            Err(async_graphql::InputValueError::expected_type(value.unwrap_or_default()))
        }
    }
    fn to_value(&self) -> async_graphql::Value {
        let mut map = async_graphql::indexmap::IndexMap::new();
        map.insert(async_graphql::Name::new("min"), InputType::to_value(&self.min));
        map.insert(async_graphql::Name::new("max"), InputType::to_value(&self.max));
        async_graphql::Value::Object(map)
    }
    fn federation_fields() -> Option<String> {
        let mut res = Vec::new();
        if let Some(fields) = <T as InputType>::federation_fields() {
            res.push({
                let res = format!("{} {}", "min", fields);
                res
            });
        } else {
            res.push(ToString::to_string("min"));
        }
        if let Some(fields) = <T as InputType>::federation_fields() {
            res.push({
                let res = format!("{} {}", "max", fields);
                res
            });
        } else {
            res.push(ToString::to_string("max"));
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
impl<T: InputType> async_graphql::InputObjectType for Range<T> {}

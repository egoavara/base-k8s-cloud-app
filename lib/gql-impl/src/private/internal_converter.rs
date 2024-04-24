use serde::de::DeserializeOwned;

pub trait InternalConverter {
    fn from_json(raw: serde_json::Value) -> Self;
    fn to_json(&self) -> serde_json::Value;
    fn from_graph(raw: async_graphql::Value) -> Self;
    fn to_graph(&self) -> async_graphql::Value;
}

macro_rules! internal_converter {
    (deref : $($t:ty),*) => {
        $(
            impl InternalConverter for $t{
                fn from_json(raw: serde_json::Value) -> Self { serde_json::from_value(raw).unwrap() }
                fn to_json(&self) -> serde_json::Value { serde_json::Value::from(*self) }
                fn from_graph(raw: async_graphql::Value) -> Self { async_graphql::from_value(raw).unwrap() }
                fn to_graph(&self) -> async_graphql::Value { async_graphql::Value::from(*self) }
            }
        )*
    };
    (clone : $($t:ty),*) => {
        $(
            impl InternalConverter for $t{
                fn from_json(raw: serde_json::Value) -> Self { serde_json::from_value(raw).unwrap() }
                fn to_json(&self) -> serde_json::Value { serde_json::Value::from(self.clone()) }
                fn from_graph(raw: async_graphql::Value) -> Self { async_graphql::from_value(raw).unwrap() }
                fn to_graph(&self) -> async_graphql::Value { async_graphql::Value::from(self.clone()) }
            }
        )*
    };
    (to_string : $($t:ty),*) => {
        $(
            impl InternalConverter for $t{
                fn from_json(raw: serde_json::Value) -> Self { <Self as std::str::FromStr>::from_str(&serde_json::from_value::<String>(raw).unwrap()).unwrap() }
                fn to_json(&self) -> serde_json::Value { serde_json::Value::from(self.to_string()) }
                fn from_graph(raw: async_graphql::Value) -> Self { <Self as std::str::FromStr>::from_str(&async_graphql::from_value::<String>(raw).unwrap()).unwrap() }
                fn to_graph(&self) -> async_graphql::Value { async_graphql::Value::from(self.to_string()) }
            }
        )*
    };
}

internal_converter!(clone : String);
internal_converter!(deref : i8, i16, i32, i64, isize);
internal_converter!(deref : u8, u16, u32, u64, usize);
internal_converter!(deref : f32, f64);
internal_converter!(deref : bool);

#[cfg(feature = "with-uuid")]
internal_converter!(to_string : uuid::Uuid);

impl InternalConverter for serde_json::Value {
    fn from_json(raw: serde_json::Value) -> Self {
        raw
    }
    fn to_json(&self) -> serde_json::Value {
        self.clone()
    }
    fn from_graph(raw: async_graphql::Value) -> Self {
        async_graphql::from_value(raw).unwrap()
    }
    fn to_graph(&self) -> async_graphql::Value {
        async_graphql::Value::from_json(self.clone()).unwrap()
    }
}

impl<T: Into<async_graphql::Value> + Into<serde_json::Value> + DeserializeOwned + Clone> InternalConverter for Vec<T> {
    fn from_json(raw: serde_json::Value) -> Self {
        serde_json::from_value(raw).unwrap()
    }
    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::from_iter(self.iter().cloned())
    }
    fn from_graph(raw: async_graphql::Value) -> Self {
        async_graphql::from_value(raw).unwrap()
    }
    fn to_graph(&self) -> async_graphql::Value {
        async_graphql::Value::from_iter(self.iter().cloned())
    }
}

impl<T: Into<async_graphql::Value> + Into<serde_json::Value> + DeserializeOwned + Clone> InternalConverter for Option<T> {
    fn from_json(raw: serde_json::Value) -> Self {
        serde_json::from_value(raw).unwrap()
    }
    fn to_json(&self) -> serde_json::Value {
        if let Some(v) = self {
            v.clone().into()
        } else {
            serde_json::Value::Null
        }
    }
    fn from_graph(raw: async_graphql::Value) -> Self {
        async_graphql::from_value(raw).unwrap()
    }
    fn to_graph(&self) -> async_graphql::Value {
        if let Some(v) = self {
            v.clone().into()
        } else {
            async_graphql::Value::Null
        }
    }
}

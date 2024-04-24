use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use async_graphql::connection::CursorType;
use async_graphql::{InputType, InputValueResult, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};

use crate::types::CursorDecodeError;

pub trait Cursor: Sized + Sync + Send + Clone {
    fn type_name() -> &'static str;

    fn decode(s: &str) -> Result<Self, CursorDecodeError>;
    fn encode(&self) -> String;
    fn chunk(&self) -> Option<CursorChunk>;

    fn with_chunk(self, chunk: CursorChunk) -> Self;
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CursorChunk {
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct CursorWrap<T>(pub T);

impl<T> CursorWrap<T> {
    pub fn new(cursor: T) -> Self {
        CursorWrap(cursor)
    }
}

impl<T: Cursor> CursorType for CursorWrap<T> {
    type Error = CursorDecodeError;

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        T::decode(s).map(CursorWrap)
    }

    fn encode_cursor(&self) -> String {
        self.0.encode()
    }
}

impl<T> Deref for CursorWrap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CursorWrap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[Scalar]
impl<T: Cursor> ScalarType for CursorWrap<T>
where
    T: Cursor,
{
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(data) => T::decode(&data).map(CursorWrap).map_err(|e| async_graphql::InputValueError::custom(e.to_string())),
            _ => Err(async_graphql::InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(T::encode(&self.0))
    }
}

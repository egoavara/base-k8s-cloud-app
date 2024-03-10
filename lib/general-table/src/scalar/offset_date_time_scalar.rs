use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use std::ops::{Deref, DerefMut};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OffsetDateTimeScalar(pub(crate) OffsetDateTime);

impl Deref for OffsetDateTimeScalar {
    type Target = OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OffsetDateTimeScalar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[Scalar]
impl ScalarType for OffsetDateTimeScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(value) => OffsetDateTime::parse(&value, &Rfc3339)
                .map_err(|_| InputValueError::custom(format!("Invalid OffsetDateTime: {}", value)))
                .map(OffsetDateTimeScalar),
            _ => Err(InputValueError::custom(format!(
                "Invalid OffsetDateTime: {}",
                value
            ))),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.format(&Rfc3339).unwrap())
    }
}

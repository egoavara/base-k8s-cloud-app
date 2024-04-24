use async_graphql::InputObject;
use gql_impl_derive::filter_crate;
use sea_query::IntoCondition;
use serde::{Deserialize, Serialize};
// // Default filter MUST have eq operation
// // Cause eq operation is used in Filter::by_id methodlll
filter_crate!(
    #[derive(Debug, Clone, Default, InputObject, Serialize, Deserialize)]
    pub struct StringFilter for String impl eq + ne + in + not_in + prefix + nprefix + contains + ncontains + suffix + nsuffix + like + nlike + regex {}

    #[derive(Debug, Clone, Default, InputObject, Serialize, Deserialize)]
    pub struct U8Filter for u8 impl eq + ne + gt + gte + lt + lte + between + nbetween { }

    #[derive(Debug, Clone, Default, InputObject, Serialize, Deserialize)]
    pub struct U16Filter for u16 impl eq + ne + gt + gte + lt + lte + between + nbetween { }

    #[derive(Debug, Clone, Default, InputObject, Serialize, Deserialize)]
    pub struct U32Filter for u32 impl eq + ne + gt + gte + lt + lte + between + nbetween { }

    #[derive(Debug, Clone, Default, InputObject, Serialize, Deserialize)]
    pub struct U64Filter for u64 impl eq + ne + gt + gte + lt + lte + between + nbetween { }

    #[derive(Debug, Clone, Default, InputObject, Serialize, Deserialize)]
    pub struct I8Filter for i8 impl eq + ne + gt + gte + lt + lte + between + nbetween { }

    #[derive(Debug, Clone, Default, InputObject, Serialize, Deserialize)]
    pub struct I16Filter for i16 impl eq + ne + gt + gte + lt + lte + between + nbetween { }

    #[derive(Debug, Clone, Default, InputObject, Serialize, Deserialize)]
    pub struct I32Filter for i32 impl eq + ne + gt + gte + lt + lte + between + nbetween { }

    #[derive(Debug, Clone, Default, InputObject, Serialize, Deserialize)]
    pub struct I64Filter for i64 impl eq + ne + gt + gte + lt + lte + between + nbetween { }
);

#[cfg(feature = "with-uuid")]
filter_crate!(
    #[derive(Debug, Clone, Default, InputObject, Serialize, Deserialize)]
    pub struct UuidFilter for uuid::Uuid impl eq + in { }
);

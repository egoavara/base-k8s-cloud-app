use async_graphql::{Enum, OneofObject};
use serde::{Deserialize, Serialize};

use gql_impl_derive::sorter_crate;

sorter_crate! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, OneofObject)]
    pub enum StringSorter for String impl asc + desc + values { }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Enum)]
    pub enum I8Sorter for i8 impl asc + desc {}
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Enum)]
    pub enum I16Sorter for i16 impl asc + desc { }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Enum)]
    pub enum I32Sorter for i32 impl asc + desc { }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Enum)]
    pub enum I64Sorter for i64 impl asc + desc { }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Enum)]
    pub enum U8Sorter for u8 impl asc + desc { }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Enum)]
    pub enum U16Sorter for u16 impl asc + desc { }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Enum)]
    pub enum U32Sorter for u32 impl asc + desc { }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Enum)]
    pub enum U64Sorter for u64 impl asc + desc { }
}


#[cfg(feature = "with-uuid")]
sorter_crate! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Enum)]
    pub enum UuidSorter for uuid::Uuid impl asc { }
}

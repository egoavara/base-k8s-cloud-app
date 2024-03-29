use table_traits_derive::sorter_internal;

sorter_internal! {
    pub enum UuidSorter for String { impl asc }

    pub enum StringSorter for String { impl asc, impl desc }

    pub enum I8Sorter for i8 { impl asc, impl desc }
    pub enum I16Sorter for i16 { impl asc, impl desc }
    pub enum I32Sorter for i32 { impl asc, impl desc }
    pub enum I64Sorter for i64 { impl asc, impl desc }
    pub enum IsizeSorter for isize { impl asc, impl desc }

    pub enum U8Sorter for u8 { impl asc, impl desc }
    pub enum U16Sorter for u16 { impl asc, impl desc }
    pub enum U32Sorter for u32 { impl asc, impl desc }
    pub enum U64Sorter for u64 { impl asc, impl desc }
    pub enum UsizeSorter for usize { impl asc, impl desc }
}

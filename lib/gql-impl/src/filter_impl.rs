
use crate::{Filter, StringFilter, U16Filter, U32Filter, UuidFilter};
pub trait FilterImpl: Sized {
    type DefaultFilter: Filter<Target = Self>;

    fn filter_by_id(&self) -> Self::DefaultFilter;
}

impl FilterImpl for String {
    type DefaultFilter = StringFilter;

    fn filter_by_id(&self) -> Self::DefaultFilter {
        StringFilter { eq: Some(self.clone()), ..Default::default() }
    }
}

#[cfg(feature = "with-uuid")]
impl FilterImpl for uuid::Uuid {
    type DefaultFilter = UuidFilter;

    fn filter_by_id(&self) -> Self::DefaultFilter {
        UuidFilter { eq: Some(*self), ..Default::default() }
    }
}

impl FilterImpl for u16 {
    type DefaultFilter = U16Filter;

    fn filter_by_id(&self) -> Self::DefaultFilter {
        U16Filter { eq: Some(*self), ..Default::default() }
    }
}
impl FilterImpl for u32 {
    type DefaultFilter = U32Filter;

    fn filter_by_id(&self) -> Self::DefaultFilter {
        U32Filter { eq: Some(*self), ..Default::default() }
    }
}

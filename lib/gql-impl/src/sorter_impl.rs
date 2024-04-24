use crate::{StringSorter, UuidSorter};
use crate::Sorter;

pub trait SorterImpl {
    type DefaultSorter: Sorter<Target = Self>;
}

impl SorterImpl for String {
    type DefaultSorter = StringSorter;
}


#[cfg(feature = "with-uuid")]
impl SorterImpl for uuid::Uuid {
    type DefaultSorter = UuidSorter;
}

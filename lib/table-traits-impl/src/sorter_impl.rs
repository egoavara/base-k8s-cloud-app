use crate::sorter_default::{StringSorter, UuidSorter};
use crate::Sorter;
use uuid::Uuid;

pub trait SorterImpl {
    type DefaultSorter: Sorter<Target = Self>;
}

impl SorterImpl for Uuid {
    type DefaultSorter = UuidSorter;
}

impl SorterImpl for String {
    type DefaultSorter = StringSorter;
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SorterKind {
    Asc,
    Desc,
    None,
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub enum SorterValue<T> {
    Asc(T),
    Desc(T),
    #[default]
    None,
    NotImplemented(SorterKind),
}



pub trait Sorter {
    type Target;

    fn implemented() -> Vec<SorterKind>;

    fn activated(&self) -> SorterKind;

    fn to_value(&self, kind: SorterKind, value: Self::Target) -> SorterValue<Self::Target>;
}

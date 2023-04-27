use crate::linq::iterator::LinqIterator;

pub trait LinqIntoIterator {
    type Item;
    type IteratorType: LinqIterator<Item = Self::Item>;

    fn into_linq_iter(self) -> Self::IteratorType;
}

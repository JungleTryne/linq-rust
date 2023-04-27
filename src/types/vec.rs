use crate::linq::into_iterator::LinqIntoIterator;
use crate::linq::iterator::LinqIterator;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct VecIterator<T> {
    data: VecDeque<T>,
}

impl<T> LinqIterator for VecIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop_front()
    }
}

impl<T> LinqIntoIterator for Vec<T> {
    type Item = T;
    type IteratorType = VecIterator<T>;

    fn into_linq_iter(self) -> Self::IteratorType {
        VecIterator {
            data: VecDeque::from(self),
        }
    }
}

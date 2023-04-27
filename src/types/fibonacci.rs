use crate::linq::iterator::LinqIterator;

pub struct FibIterator {
    first: u32,
    second: u32,
}

impl FibIterator {
    pub fn new() -> Self {
        Self {
            first: 0,
            second: 1
        }
    }
}

impl LinqIterator for FibIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let sum = self.first + self.second;
        self.first = self.second;
        self.second = sum;
        Some(sum)
    }
}

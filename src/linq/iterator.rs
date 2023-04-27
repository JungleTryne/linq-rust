use std::collections::BTreeMap;
use std::fmt::Debug;

pub trait LinqIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn select<Func>(self, func: Func) -> Select<Self, Func>
    where
        Self: Sized,
    {
        Select::new(self, func)
    }

    fn where_linq<Pred>(self, pred: Pred) -> Where<Self, Pred>
    where
        Self: Sized,
    {
        Where::new(self, pred)
    }

    fn take(self, k: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take::new(self, k)
    }

    fn flatten(self) -> Flatten<Self>
    where
        Self: Sized,
    {
        Flatten::new(self)
    }

    fn order_by<F>(self, key: F) -> OrderBy<Self, F>
    where
        Self: Sized,
    {
        OrderBy::new(self, key)
    }

    fn group_by<F, K>(self, key: F) -> GroupBy<Self, F, K>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> K,
    {
        GroupBy::new(self, key)
    }

    fn to_list(mut self) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        let mut result = vec![];
        while let Some(el) = self.next() {
            result.push(el)
        }
        result
    }
}

/// Select

pub struct Select<I, Func> {
    iter: I,
    func: Func,
}

impl<I, Func> Select<I, Func> {
    pub fn new(iter: I, func: Func) -> Self {
        Self { iter, func }
    }
}

impl<Output, I, Func> LinqIterator for Select<I, Func>
where
    I: LinqIterator,
    Func: Fn(I::Item) -> Output,
{
    type Item = Output;

    fn next(&mut self) -> Option<Output> {
        Some((self.func)(self.iter.next()?))
    }
}

/// Where

pub struct Where<I, Pred> {
    iter: I,
    pred: Pred,
}

impl<I, Pred> Where<I, Pred> {
    pub fn new(iter: I, pred: Pred) -> Self {
        Self { iter, pred }
    }
}

impl<I, Pred> LinqIterator for Where<I, Pred>
where
    I: LinqIterator,
    Pred: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_elem = self.iter.next()?;
        while !(self.pred)(&next_elem) {
            next_elem = self.iter.next()?;
        }
        Some(next_elem)
    }
}

/// Take

pub struct Take<I> {
    iter: I,
    k: usize,
    current: usize,
}

impl<I> Take<I> {
    pub fn new(iter: I, k: usize) -> Self {
        Self {
            iter,
            k,
            current: 0,
        }
    }
}

impl<I> LinqIterator for Take<I>
where
    I: LinqIterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.k {
            return None;
        }
        self.current += 1;
        self.iter.next()
    }
}

/// Flatten

pub struct Flatten<I>
where
    I: LinqIterator,
{
    iter: I,
    current_sub_iter: Option<I::Item>,
}

impl<I> Flatten<I>
where
    I: LinqIterator,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            current_sub_iter: None,
        }
    }
}

impl<I> LinqIterator for Flatten<I>
where
    I: LinqIterator,
    <I as LinqIterator>::Item: LinqIterator,
{
    type Item = <I::Item as LinqIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sub_iter.is_none() {
            self.current_sub_iter = self.iter.next();
            if self.current_sub_iter.is_none() {
                return None;
            }
        }

        let next_element = self.current_sub_iter.as_mut().unwrap().next();
        if next_element.is_none() {
            self.current_sub_iter = self.iter.next();
            return if self.current_sub_iter.is_none() {
                None
            } else {
                self.current_sub_iter.as_mut().unwrap().next()
            }
        }

        next_element
    }
}

/// OrderBy

pub struct OrderBy<I, F>
where
    I: LinqIterator,
{
    iter: I,
    key: F,
    container: Vec<I::Item>,
    sorted: bool,
}

impl<I, F> OrderBy<I, F>
where
    I: LinqIterator,
{
    pub fn new(iter: I, key: F) -> Self {
        Self {
            iter,
            key,
            container: Vec::new(),
            sorted: false,
        }
    }
}

impl<I, F, Key> LinqIterator for OrderBy<I, F>
where
    I: LinqIterator,
    F: Fn(&I::Item) -> Key,
    Key: Ord,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.sorted {
            self.sorted = true;
            while let Some(el) = self.iter.next() {
                self.container.push(el);
            }
            self.container.sort_by_key(&self.key);
            self.container.reverse();
        }

        self.container.pop()
    }
}

/// GroupBy

pub struct GroupBy<I, F, K>
where
    I: LinqIterator,
    F: Fn(&I::Item) -> K,
{
    iter: I,
    key: F,
    initialized: bool,
    container: Option<Vec<(K, Vec<I::Item>)>>,
}

impl<I, F, K> GroupBy<I, F, K>
where
    I: LinqIterator,
    F: Fn(&I::Item) -> K,
{
    pub fn new(iter: I, key: F) -> Self {
        Self {
            iter,
            key,
            initialized: false,
            container: None,
        }
    }
}

impl<I, F, K> LinqIterator for GroupBy<I, F, K>
where
    I: LinqIterator,
    F: Fn(&I::Item) -> K,
    K: Ord + Debug,
    I::Item: Debug,
{
    type Item = (K, Vec<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.initialized {
            self.initialized = true;
            let mut map = BTreeMap::<K, Vec<I::Item>>::new();
            while let Some(elem) = self.iter.next() {
                let key = (self.key)(&elem);
                if map.contains_key(&key) {
                    map.get_mut(&key).unwrap().push(elem);
                } else {
                    map.insert(key, vec![elem]);
                }
            }

            let mut container: Vec<(K, Vec<I::Item>)> = Vec::new();
            for (key, value) in map {
                container.push((key, value))
            }
            container.reverse();

            self.container = Some(container);
            return self.container.as_mut().unwrap().pop();
        }

        self.container.as_mut().unwrap().pop()
    }
}

use crate::linq::into_iterator::LinqIntoIterator;
use crate::linq::iterator::LinqIterator;
use crate::types::fibonacci::FibIterator;

pub mod linq;
mod types;

fn main() {
    let fib = FibIterator::new()
        .where_linq(|x: &u32| {
            return x % 3 == 0
        })
        .select(|x: u32| {
            return if x % 2 == 0 {
                x * x
            } else {
                x
            }
        })
        .take(5)
        .to_list();

    let word_count = vec![
        "hello world".to_owned(),
        "world hello my dear friend".to_owned(),
        "this is world another line".to_owned(),
        "to test count words".to_owned(),
    ].into_linq_iter()
        .select(|x: String| -> Vec<String> {
            x.split_whitespace().map(str::to_string).collect()
        })
        .select(|x: Vec<String>| {
            x.into_linq_iter()
        })
        .flatten()
        .select(|x: String| {
            (x, 1)
        })
        .group_by(|x: &(String, u32)| x.0.clone())
        .select(|(key, values): (String, Vec<_>)| {
            (key, values.len() as u32)
        })
        .to_list();

    println!("Fibonacci: {:?}", fib);
    println!("Word count: {:?}", word_count);
}

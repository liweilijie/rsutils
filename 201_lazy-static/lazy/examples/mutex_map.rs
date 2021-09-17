//! This example shows how to wrap a data structure in a mutex to achieve safe mutability.
extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref MUTEX_MAP: Mutex<HashMap<u32, &'static str>> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        Mutex::new(m)
    };

    static ref COUNT: usize = MUTEX_MAP.lock().unwrap().len();
    static ref NUMBER: u32 = times_two(21);
}

fn times_two(n: u32) -> u32 { n * 2 }

fn main() {
    println!("The before map has {} entries.", *COUNT);
    MUTEX_MAP.lock().unwrap().insert(0, "boo");
    println!(
        "The entry for `0` is `\"{}\".",
        MUTEX_MAP.lock().unwrap().get(&0).unwrap()
    );
    println!("The after map has {} entries.", *COUNT);
    println!("A expensive calculation on a static results in: {}.", *NUMBER);
}
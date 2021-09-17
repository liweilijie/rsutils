# lazy_static
这个库主要用于创建全局可变变量的。

## 创建一个全局的可变vector

```rust
extern crate lazy_static;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("called {}", ARRAY.lock().unwrap().len());

}
```

## 创建一个可变的HashMap

```rust

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
```

然而，全局静态变量是我们经常使用的，必须找到高效的方式来创建和查找全局静态变量。下面我们讲述如何在rust中使用全局静态变量。
请看color.rs代码


## 链接

- [lazy_static](https://blog.csdn.net/wsp_1138886114/article/details/109612557)

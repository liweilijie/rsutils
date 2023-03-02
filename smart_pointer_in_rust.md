# 智能指针

我们之前简单介绍过指针，这里还是先回顾一下：指针是一个持有内存地址的值，可以通过解引用来访问它指向的内存地址，理论上可以解引用到任意数据类型；引用是一个特殊的指针，它的解引用访问是受限的，只能解引用到它引用数据的类型，不能用作它用。

那什么是智能指针呢？

在指针和引用的基础上，Rust 偷师 C++，提供了智能指针。智能指针是一个表现行为很像指针的数据结构，但除了指向数据的指针外，它还有元数据以提供额外的处理能力。这个定义有点模糊，我们对比其他的数据结构来明确一下。你有没有觉得很像之前讲的胖指针。智能指针一定是一个胖指针，但胖指针不一定是一个智能指针。比如 &str 就只是一个胖指针，它有指向堆内存字符串的指针，同时还有关于字符串长度的元数据。

String 除了多一个 capacity 字段，似乎也没有什么特殊。**但 String 对堆上的值有所有权，而 &str 是没有所有权的，这是 Rust 中智能指针和普通胖指针的区别。**

所以再清晰一下定义，**在 Rust 中，凡是需要做资源回收的数据结构，且实现了 Deref/DerefMut/Drop，都是智能指针。**

以下这些结构都是智能指针：

- String
- Box<T>
- Vec<T>
- Rc<T>, Arc<T>
- Ref<T>, RefMut<T>, RefCell<T>
- PathBuf
- Cow<'a, B>
- MutexGuard<T>, RwLockReadGuard<T>, RwLockWriteGuard

## Box<T>
在Rust中，所有值默认都是栈上分配。通过创建Box<T>，可以把值装箱，使它在堆上分配。
Box<T>类型是一个智能指针，因为它实现了`Deref trait`，它允许Box<T>值被当作引用对待。当Box<T>值离开作用域时，由于它实现了`Drop trait`，首先删除其指向的堆数据，然后删除自身。

Deref这个trait, 允许我们重载解引用运算符*。实现Deref的智能指针可以被当作引用来对待，也就是说可以对智能指针使用*运算符进行解引用。

Box<T>对Deref的实现：
```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &**self
    }
}
```
该实现返回`&**self`。为什么呢？由于`self`是一个`&Box<T>`，因此对其进行一次解引用`*`将获得一个`Box<T>`，而第二次解引用`*`将获得一个`T`。最后，将其包装在引用`&`中并返回。

注：如果是我们自定义的类型，要实现deref，则不能仿照它，否则会造成无限递归。

Box<T>是堆上分配的指针类型，称为“装箱”（boxed），其指针本身在栈，指向的数据在堆，在Rust中提供了最简单的堆分配类型。使用Box<T>的情况：

递归类型和trait对象。Rust需要在编译时知道一个类型占用多少空间，Box<T>的大小是已知的。
“大”的数据转移所有权。用Box<T>只需拷贝指针。

递归类型的经典示例：
```rust
use List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let recursive_list: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:?}", recursive_list); // 打印出: Cons(1, Cons(2, Nil))
}
```

trait对象的示例：
```rust
trait T {
    fn m(&self) -> u64;
}

struct S {
    i: u64
}

impl T for S {
    fn m(&self) -> u64 { self.i }
}

fn f(x: Box<dyn T>) {
    println!("{}", x.m())
}

fn main() {
    let s = S{ i: 100 };
    println!("{}", s.m());

    let b: Box<S> = Box::new(S{i:100});
    f(b); // 动态调用
}
```

自定义的类型的Deref实例：
```rust
use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T> {
    value: T,
}

impl<T> Deref for MyBox<T> {
    type Target = T;

fn deref(&self) -> &Self::Target {
    &self.value
  }
}

fn main() {
    let instance = MyBox{value: 10};
    assert_eq!(10, *instance);
    println!("{}, {}", *instance, *(instance.deref()));
}
```

## Cow<'a, B>
Cow 是 Rust 下用于提供写时克隆（Clone-on-Write）的一个智能指针，它跟虚拟内存管理的写时复制（Copy-on-write）有异曲同工之妙：**包裹一个只读借用，但如果调用者需要所有权或者需要修改内容，那么它会 clone 借用的数据。**

我们看Cow的定义：
```rust
pub enum Cow<'a, B>
where
    B: 'a + ToOwned, + ?Sized
{
    /// Borrowed data.
    Borrowed(&'a B),
    /// Owned data.
    Owned(<B as ToOwned>::Owned),
}
```

它是一个 enum，可以包含一个对类型 B 的只读引用，或者包含对类型 B 的拥有所有权的数据。

这里又引入了两个 trait，首先是 ToOwned，在 ToOwned trait 定义的时候，又引入了 Borrow trait，它们都是 std::borrow 下的 trait：
```rust
pub trait ToOwned {
    type Owned: Borrow<Self>;
    #[must_use = "cloning is often expensive and is not expected to have side effects"]
    fn to_owned(&self) -> Self::Owned;

    fn clone_into(&self, target: &mut Self::Owned) { ... }
}

pub trait Borrow<Borrowed> where Borrowed: ?Sized {
    fn borrow(&self) -> &Borrowed;
}
```

实现的原理很简单，根据 self 是 Borrowed 还是 Owned，我们分别取其内容，生成引用：对于 Borrowed，直接就是引用；对于 Owned，调用其 borrow() 方法，获得引用。这就很厉害了。虽然 Cow 是一个 enum，但是通过 Deref 的实现，我们可以获得统一的体验，比如 Cow<str>，使用的感觉和 &str / String 是基本一致的。注意，**这种根据 enum 的不同状态来进行统一分发的方法是第三种分发手段**，之前讲过可以使用泛型参数做静态分发和使用 trait object 做动态分发。

**减少不必要的堆内存分配是提升系统效率的关键手段。**

Cow的例子

```toml
[dependencies]
serde = { version = "1.0.130", features = ["derive"] }
serde_json = "1.0.68"
url = "2.2.2"
```

```rust
use std::borrow::Cow;
use url::Url;

fn main() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = url.query_pairs();
    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    // 因为 k, v 都是 Cow 他们用起来感觉和 &str 或者 String 一样
    // 此刻，他们都是 Borrowed
    println!("key: {}, v: {}", k, v); // key: page, v: 1024
    k.to_mut().push_str("_lala"); // 当修改发生时，k 变成 Owned
    print_pairs((k, v)); // key: Owned page_lala, value: Borrowed 1024

    print_pairs(pairs.next().unwrap()); // key: Borrowed sort, value: Borrowed desc

    // 在处理 extra=hello%20world 时，value 被处理成 "hello world"
    // 所以这里 value 是 Owned
    print_pairs(pairs.next().unwrap()); // key: Borrowed extra, value: Owned hello world

    // 示例2: serde

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct User<'input> {
        #[serde(borrow)]
        name: Cow<'input, str>,
        age: u8,
    }

    let input = r#"{"name": "Tyr", "age": 18}"#;
    // 我们可以通过如下代码将一个 JSON 数据反序列化成 User 类型，
    // 同时让 User 中的 name 使用 Cow 来引用 JSON 文本中的内容
    let user: User = serde_json::from_str(input).unwrap();

    // 打印: borrowed Tyr
    match user.name {
        Cow::Borrowed(x) => println!("borrowed {}", x),
        Cow::Owned(x) => println!("owned {}", x),
    }
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed {}", v),
        Cow::Owned(v) => format!("Owned {}", v),
    }
}
```

## MutexGuard<T>
如果说，上面介绍的 String、Box<T>、Cow<'a, B> 等智能指针，都是通过 Deref 来提供良好的用户体验，那么 MutexGuard<T> 是另外一类很有意思的智能指针：它不但通过 Deref 提供良好的用户体验，**还通过 Drop trait 来确保，使用到的内存以外的资源在退出时进行释放。**

我们来看一个使用 Mutex 和 MutexGuard 的例子
```rust
use lazy_static::lazy_static;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// lazy_static 宏可以生成复杂的 static 对象
lazy_static! {
    // 一般情况下 Mutex 和 Arc 一起在多线程环境下提供对共享内存的使用
    // 如果你把 Mutex 声明成 static，其生命周期是静态的，不需要 Arc
    static ref METRICS: Mutex<HashMap<Cow<'static, str>, usize>> = Mutex::new(HashMap::new());
}

fn main() {
    // 用 Arc 来提供并发环境下的共享所有权（使用引用计数）
    let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));

    for _ in 0..32 {
        let m = metrics.clone();
        thread::spawn(move || {
            let mut g = m.lock().unwrap();
            // 此时只有拿到 MutexGuard 的线程可以访问 HashMap
            let data = &mut *g;
            // Cow 实现了很多数据结构的 From trait，
            // 所以我们可以用 "hello".into() 生成 Cow
            let entry = data.entry("hello".into()).or_insert(0);
            *entry += 1;
        });
    }

    thread::sleep(Duration::from_millis(100));

    println!("metrics:{:?}", metrics.lock().unwrap()); // metrics:{"hello": 32}
}
```
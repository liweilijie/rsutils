# FromStr trait基础知识学习

今天来学习rust中的`std::str::FromStr`这个**trait**。

FromStr是rust标准库中定义的trait，当一个类型实现FromStr trait后，调用字符串的泛型函数`str.parse()`就可以很方便的实现字符串到某个具体类型的转换。 注意FromStr没有生命周期参数，因此只能解析本身不包含生命周期的类型。例如，可以解析实现FromStr的i32，但不能解析&i32，可以解析包含i32的struct，但不能解析包含&i32的struct。

FromStr trait的定义如下:
```rust
pub trait FromStr: Sized {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err>;
}
```

FromStr包含了一个关联类型Err和一个函数签名`fn from_str(s: &str) -> Result<Self, Self::Err>`。

以下是rust标准库文档中关于FromStr的例子:
```rust
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(',')
                                 .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

fn main() {
  let p1 = Point::from_str("(2,4)");
  let p2 = "(4,2)".parse::<Point>();
  println!("{:?} {:?}", p1, p2);
  // Ok(Point {x:2,y:4}) Ok(Point{x:4,y:2}
}
```

可以直接调用Point::from_str完成从字符串到具体类型的转换，也可以通过str.parse隐式调用FromStr的from_str，经常使用的是隐式的方式。

通过实现std::str::FromStr trait，为某个类型提供从字符串到具体类型转换的功能在日常开发和一些第三方库中被频繁用到，例如用来处理mime类型的第三方库mime (https://docs.rs/mime)中的Mime struct就实现了FromStr。这样就可以直接将字符串解析为Mime类型，进行后续不同mime类型的处理逻辑:


```rust
let plain_text: mime::Mime = "text/plain".parse().unwrap();
assert_eq!(plain_text, mime::TEXT_PLAIN);
```
## 参考

- [https://doc.rust-lang.org/std/str/trait.FromStr.html](https://doc.rust-lang.org/std/str/trait.FromStr.html)
- [rust的FromStr](https://blog.frognew.com/2020/07/rust-fromstr-trait.html)

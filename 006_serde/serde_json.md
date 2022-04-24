**Serde 是一个， *序列*化 和 *反*序列化 Rust 数据结构，的有效且通用框架**

### 目录

<!-- START doctoc -->
<!-- END doctoc -->

## Cargo.toml

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

您可能正在寻找:

- [JSON API 文档](https://docs.serde.rs/serde_json/)
- [Serde API 文档](https://docs.serde.rs/serde/)
- [关于 Serde 的详细文档](https://serde.rs/)
- [配置`#[derive(Serialize, Deserialize)]`](https://serde.rs/codegen.html)
- [Release 告示](https://github.com/serde-rs/json/releases)

JSON 是一种无处不在的开放标准格式,它使用人类可读的文本来传输，键值对组成的数据对象.

```json,ignore
{
  "name": "John Doe",
  "age": 43,
  "address": {
    "street": "10 Downing Street",
    "city": "London"
  },
  "phones": ["+44 1234567", "+44 2345678"]
}
```

您可能会发现自己需要在 Rust 中使用 JSON 数据的三种常见方式。

- **作为文本数据.**您在 HTTP 端点上，接收到未处理的 JSON 数据字符串,从文件读取或是准备发送到远程服务器。
- **作为无类型或松散类型的表示.**也许你想在传递它之前，检查一些 JSON 数据是否有效,但是不知道它包含的内容的结构。或是您想要进行非常基本的操作,例如在特定位置插入一个key字段。
- **作为强类型的 Rust 数据结构.**当您希望所有或大部分数据符合特定结构并，希望在 JSON 非松散结构绊脚的情况下，完成真正的工作。

Serde JSON 提供了高效,灵活,安全的方法,并为它们每个表达式之间转换数据。

## 对无类型的 JSON 值进行操作

可以在以下，枚举泛型中操纵任何有效的 JSON 数据。这个数据结构是[`serde_json::Value`][value].

```rust,ignore
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

通过[`serde_json::from_str`][from_str]函数，可以将一串 JSON 数据解析为`serde_json::Value`。还有[`from_slice`][from_slice]用于字节切片&[u8]的解析，和[`from_reader`][from_reader]用于解析任意`io::Read`，像一个 File 或 TCP 流。

<a href="https://play.rust-lang.org/?gist=a266662bc71712e080efbf25ce30f306" target="_blank">
<img align="right" width="50" src="https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/img/run.png">
</a>

```rust
extern crate serde_json;

use serde_json::{Value, Error};

fn untyped_example() -> Result<(), Error> {
    // 一些JSON的输入数据作为 一个 &str。因可能来自用户定义.
    let data = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;

    // 将数据字符串解析为serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // 得到 部分数据 , 通过 方括号中的索引.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
```

像`v["name"]`这样的方括号索引的结果，就是对该索引数据的借用,所以类型是`&Value`。可以使用 **字符串字段** 来索引 JSON map，而JSON 数组可以使用**整数字段**索引。如果数据的类型不适合索引的类型,或者如果映射不包含要索引的键,或者Vec数组中的索引超出范围,则返回`Value::Null`元素.

当一个`Value`打印时,它打印为 JSON 字符串.所以在上面的代码中,输出看起来像`Please call "John Doe" at the number "+44 1234567"`。出现引号是因为`v["name"]`是一个包含 JSON 字符串及其 JSON 表达式`"John Doe"`的`&Value`。要打印没有引号的纯字符串，其中涉及JSON 字符串用[`as_str()`]转换为 Rust 字符串，或如下一节所述，要避免使用`Value`。

[`as_str()`]: https://docs.serde.rs/serde_json/enum.Value.html#method.as_str

运用`Value`枚举，对非常基本的任务来说，已经足够了，但是对于任何更重要的任务来说是单调了些的。错误处理很难正确实现,例如试想一下,试图检测输入数据中，是否存在无法识别的字段。当你犯错误时,编译器无法帮助你,如拼写错误: `v["name"]`变成了`v["nmae"]`，在您的代码中，像这样的索引，可能不下几十处吧。

## 将 JSON 解析为强类型数据结构

Serde 提供了一种将 JSON 数据，自动映射到 Rust 数据结构的强大方法.

<a href="https://play.rust-lang.org/?gist=cff572b80d3f078c942a2151e6020adc" target="_blank">
<img align="right" width="50" src="https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/img/run.png">
</a>

```rust
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<(), Error> {
    // 一些JSON的输入数据作为 一个 &str。因可能来自用户定义.
    let data = r#"{
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                      "+44 1234567",
                      "+44 2345678"
                    ]
                  }"#;

    // 将数据字符串  解析 到一个 Person对象 . 
    // 这与上面生成 serde_json::Value 的函数完全相同, 但
    // 现在 我们告诉它以 一个Person 类型返回。
    let p: Person = serde_json::from_str(data)?;

    // 就像对待 其他Rust数据结构一样。
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
```

这里的`serde_json::from_str`像以前一样运行，但这次我们赋予了返回值一个`Person`类型变量，所以 Serde 会自动将输入数据解释为一个`Person`，如果布局不符合`Person`预计看起来那样，则产生信息性错误消息.

任何实现 Serde `Deserialize` trait 的类型，都可以通过这种方式反序列化。这包括内置的 Rust 标准库类型`Vec<T>`和`HashMap<K, V>`，以及任何带注释`#[derive(Deserialize)]`的结构或枚举。

一旦我们有了`Person`类型的`p`,我们的 IDE 和 Rust 编译器可以帮助我们正确使用它，就像它们对任何其他 Rust 代码一样。IDE 可以自动填充字段名称，以防止打字错误, 而这对`serde_json::Value`来说是不可能的。Rust 编译器可以在我们编写`p.phones[0]`时，检查它, 然后保证`p.phones`是一个`Vec<String>`，所以索引到它是有道理的,并返回一个`String`。

## 构造 JSON 值

Serde JSON 提供了一个[`json!`宏][macro]，以使建立JSON 语法的对象`serde_json::Value`变得非常自然。为了使用这个宏,`serde_json`需要，加上导入宏的`#[macro_use]`属性，否则[`json!`宏][macro]是没法使用到的。

<a href="https://play.rust-lang.org/?gist=c216d6beabd9429a6ac13b8f88938dfe" target="_blank">
<img align="right" width="50" src="https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/img/run.png">
</a>

```rust
#[macro_use]
extern crate serde_json;

fn main() {
    // `john` 类型是 `serde_json::Value`
    let john = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // 将 JSON 字符串 转换 并 打印出来
    println!("{}", john.to_string());
}
```

`Value::to_string()`函数会将一个`serde_json::Value`转换成一个`String`JSON 文本。

一个巧妙的事情`json!`宏，是可以在构建 JSON 值时,将变量和表达式直接插入到 JSON 值中。Serde 将在编译时，检查您插入的值是否能够表示为 JSON.

<a href="https://play.rust-lang.org/?gist=aae3af4d274bd249d1c8a947076355f2" target="_blank">
<img align="right" width="50" src="https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/img/run.png">
</a>

```rust
let full_name = "John Doe";
let age_last_year = 42;

//  `john`类型是 `serde_json::Value`
let john = json!({
  "name": full_name,
  "age": age_last_year + 1,
  "phones": [
    format!("+44 {}", random_phone())
  ]
});
```

这非常方便,但我们以前有遇到一个`Value`问题就，是如果我们获得错误,IDE 和 Rust 编译器无法帮助我们。Serde JSON 提供了一种将强类型数据结构，序列化为 JSON 文本的更好方法.

## 通过序列化数据结构来创建 JSON

可以用[`serde_json::to_string`][to_string]，将数据结构转换为 JSON 字符串。还有[`serde_json::to_vec`][to_vec]能序列化成一个`Vec<u8>`和，[`serde_json::to_writer`][to_writer]会序列化为任意`io::Write`，例如一个 File 或 TCP 流.

<a href="https://play.rust-lang.org/?gist=40967ece79921c77fd78ebc8f177c063" target="_blank">
<img align="right" width="50" src="https://raw.githubusercontent.com/serde-rs/serde-rs.github.io/master/img/run.png">
</a>

```rust
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> Result<(), Error> {
    // 一些数据结构
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // 序列化成一个 JSON string.
    let j = serde_json::to_string(&address)?;

    // 然后，你就可以，打印, /写入文件 / 还能发送到HTTP服务器
    println!("{}", j);

    Ok(())
}
```

任何实现 Serde `Serialize` trait 的类型，都可以这种方式序列化.这包括内置的 Rust 标准库类型`Vec<T>`和`HashMap<K, V>`,以及任何带注释`#[derive(Serialize)]`的结构或枚举.

## 性能

它很快。根据数据的特征,您应该期望在每秒 500 到 1000 MB的反序列化和每秒 600 到 900 MB的序列化。这能与最快的 C 和 C ++ JSON 库竞争乐，甚至在许多用例，甚至快上 30%。基准们生活在[serde-rs/json-benchmark]repo.

[serde-rs/json-benchmark]: https://github.com/serde-rs/json-benchmark

## 获得帮助

Serde 开发人员住在[`irc.mozilla.org`](https://wiki.mozilla.org/IRC)的 #serde 频道上。#rust 频道也是一个很好的资源,响应时间通常较快,但对 Serde 的了解较少。如果 IRC 不是你的主要方式，我们用于很乐意回应[GitHub 问题](https://github.com/serde-rs/json/issues/new).

## 没有标准的支持

这个箱子目前需要 Rust 标准库。对于 Serde 非标准库的的 JSON 支持,请参阅[`serde-json-core`]箱.

[`serde-json-core`]: https://japaric.github.io/serde-json-core/serde_json_core/

## 执照

Serde JSON 根据任何一种许可

- Apache License,Version 2.0,([许可证 APACHE](LICENSE-APACHE)要么<http://www.apache.org/licenses/LICENSE-2.0>)
- MIT 许可证([LICENSE-MIT](LICENSE-MIT)要么<http://opensource.org/licenses/MIT>)

根据你的选择.

### 贡献

除非您明确说明,否则您按照 Apache-2.0 许可证的规定有意提交包含在 Serde JSON 中的任何贡献应按上述方式进行双重许可,不附加任何其他条款或条件.

[value]: https://docs.serde.rs/serde_json/value/enum.Value.html
[from_str]: https://docs.serde.rs/serde_json/de/fn.from_str.html
[from_slice]: https://docs.serde.rs/serde_json/de/fn.from_slice.html
[from_reader]: https://docs.serde.rs/serde_json/de/fn.from_reader.html
[to_string]: https://docs.serde.rs/serde_json/ser/fn.to_string.html
[to_vec]: https://docs.serde.rs/serde_json/ser/fn.to_vec.html
[to_writer]: https://docs.serde.rs/serde_json/ser/fn.to_writer.html
[macro]: https://docs.serde.rs/serde_json/macro.json.html
# serde
serde 用得非常多，这里举一些常用的个例。 先来定义一下什么叫序列化和反序列化
- 序列化： 将数据结构或对象转换成二进制串的过程 `Serialize`
- 反序列化：将在序列化过程中所生成的二进制串转换成数据结构或者对象的过程 `Deserialize`

## 序列化
 支持的类型有：`String, &str, usize, Vec<T>, HashMap<K, V>`.

```rust
// Convert the Point to a JSON string.
let serialized = serde_json::to_string(&point).unwrap();

// Convert the JSON string back to a Point.
let deserialized: Point = serde_json::from_str(&serialized).unwrap();
```

## 对无类型的 JSON 值进行操作
通过`serde_json::from_str`函数，可以将一串 JSON 数据解析为`serde_json::Value`。
还有`from_slice`用于字节切片`&[u8]`的解析，和`from_reader`用于解析任意`io::Read`，像一个 File 或 TCP 流。

## 构造 JSON值
使用 `json!` 可以构造一个 JSON 值，例如：

```rust
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
```
`Value::to_string()`函数会将一个`serde_json::Value`转换成一个`String` JSON 文本。


一个巧妙的事情`json!`宏，是可以在构建`JSON`值时,将变量和表达式直接插入到`JSON`值中。
Serde 将在编译时，检查您插入的值是否能够表示为 JSON.

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

## 容器属性

- #[serde(rename = "name")]
    - 使用给定的名字而不是其Rust名
    - 同时允许如下写法
    - #[serde(rename(serialize = "ser_name"))]
    - #[serde(rename(deserialize = "de_name"))]
    - #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]





## 参考

- [https://serde.rs](https://serde.rs)
- [https://magiclen.org/rust-serde/](https://magiclen.org/rust-serde/)
- [serde_json-zh](https://github.com/chinanf-boy/serde_json-zh)
- [ 官翻译](https://www.rectcircle.cn/posts/rust-serde/)


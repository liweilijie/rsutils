# rust常用知识点

clap 的默认解析函数功能`#[arg(value_parser = parse_kv_pair)]` 请看[httpie](httpie).

`Some(T)/None` 转化为 `Ok(T)/Err(E)` 然后使用?处理错误返回的例子请看[httpie](httpie).

## trait

### FromStr

[trait.FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html)

- FromStr: s.parse(); 我们为 KvPair 结果体实现了 FromStr, 则我们可以使用字符串的 s.parse()来得到 KvPair

- [函数式编程.适配器](combinators)
- [异步适配器.concurrent_stream](concurrent_stream)
- [异步适配器.tricoder](tricoder)
- [cli命令行工具.httpie](httpie)
- [图片处理服务器.thumbor](thumbor)

# httpie

查看代码行数:

```bash
cargo install tokie
tokie src/main.rs
```

## 测试

测试结果, 测试示例
```bash
cargo test
cargo build --quiet && target/debug/httpie post https://httpbin.org/post a=1 b=2
```
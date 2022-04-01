# arc_swap
热加载配置文件的方法。适用于写少读多的场景。

主要使用到了 ArcSwap结构的三个方法
```rust
// 初始化
let config = ArcSwap::from(Arc::new(String::default()));

// 创建一个新的值
let new_conf = Arc::new("New configuration".to_owned());
config.store(new_conf);

// 获取当前值
let cfg = config.load();
if !cfg.is_empty() {
    assert_eq!(**cfg, "New configuration");
    return;
}
```

## 测试

```bash
# 读取值
http get http://127.0.0.1:3000/
# 修改好文件之后，再次加载文件内容
http post http://127.0.0.1:3000/reload
```
# log & env_logger


## default 实现
default实现的方式：
```rust
cargo add log
cargo add env_logger

[dependencies]
env_logger = "0.9.0"
log = "0.4.14"
```

这两个`creates`顾名思义是用来打印日志的库。简单封装一下初始化函数，在程序一开始调用即可：

```rust
pub fn init_logger() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
}
```

在需要打印日志的地方利用`log` crate提供的宏即可输出日志：

宏的用法与`println`宏的用法一致。

```rust
trace!("some trace log {}", 1);
debug!("some debug log");
info!("some information log");
warn!("some warning log");
error!("some error log");
```

## 自定义打印日志的格式

```rust
fn init_logger() {
    use std::io::Write;
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
    let mut builder = env_logger::Builder::from_env(env);
    builder
        .format(|buf, record| {
            // 彩色打印日志级别
            let level = { buf.default_styled_level(record.level()) };
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level,
                record.module_path().unwrap_or("<unnamed>"),
                record.line().unwrap_or(0),
                &record.args()
            )
        })
        .init();

    info!("env_logger initialized.");
}
```

 输出结果：

 ```text
[2022-03-03 16:59:26 INFO custom_default_format:59] env_logger initialized.
[2022-03-03 16:59:26 DEBUG custom_default_format:66] some debug log
[2022-03-03 16:59:26 INFO custom_default_format:67] some information log
[2022-03-03 16:59:26 WARN custom_default_format:68] some warning log
[2022-03-03 16:59:26 ERROR custom_default_format:69] some error log
 ```



## 参考

- [http://zzdirty.space/2021/05/28/rust-frp-2/](http://zzdirty.space/2021/05/28/rust-frp-2/)
- [https://blog.csdn.net/weixin_33918114/article/details/87943290](https://blog.csdn.net/weixin_33918114/article/details/87943290)

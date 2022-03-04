# 代码功能

iptables 的 prometheus exporter 功能，需要 sudo权限运行。


`Result.map_err()`: 通过对包含的`Err`值应用函数，将`Ok`值 `Maps` 转换为`Result<T, F>`，而保持`Ok`值不变。 此函数可用于在处理错误时传递成功的结果。
```rust
macro_rules! unwrap_or_exit {
    ($e:expr) => {{
        use std::process;
        use tracing::error;
        ($e).map_err(|e| {
            error!("{}", e);
            eprintln!("error: {}", e);
            process::exit(1);
        })
        .unwrap()
    }};
}
```
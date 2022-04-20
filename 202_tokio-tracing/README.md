# tracing
打印日志, 安装依赖：
```bash
cargo add tracing
cargo add tracing-subscriber --features=env-filter
```


## 常用的打印所有的日志记录

意思是所有比`TRACE`更高等级:`info`,`warn`,`debug`,`error`等都可以打印出来的意思。
```rust
use tracing::{debug, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {

    // 1. 简单的方式跑起来
    tracing_subscriber::fmt().init();
    // 2. 简单的方式跑起来
	tracing_subscriber::fmt()
		 // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
		 // will be written to stdout.
		 .with_max_level(tracing::Level::TRACE)
		 // sets this to be the default, global collector for this application.
		 .init();

    // 3. 常用的方式
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "strangers=debug".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init(); 
}
```

## 修改东8区并且自定义时间格式

需要引入的库

```toml
time = "0.3.9"
tracing = "0.1.34"
tracing-subscriber = {version = "0.3.11", features = ["env-filter", "time", "local-time"]}
```

然后代码如下：

```rust

use time::macros::format_description;
use time::UtcOffset;
use tracing_subscriber::fmt::time::OffsetTime;

fn main() {
    // 第三种, 自定义时间显示格式
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "tracinglog=debug".to_string()),
        ))
        .with_timer(local_time)
        .init();

    info!("info.");
    warn!("warn.");
    debug!("debug.");
    trace!("trace.");
}
```

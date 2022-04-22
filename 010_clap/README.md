# clap

[https://github.com/clap-rs/clap](https://github.com/clap-rs/clap)

`command` 方式创建简单命令

依赖：
```toml
clap = {version = "3.1.10", features = ["cargo"] }
```

代码：

```rust
use anyhow::{anyhow, Result};
use clap::arg;

fn main() -> Result<()> {
    let matches = clap::command!()
        .arg(arg!(path: -p).required(true).help("Set the glob file path.").takes_value(true))
        .arg(arg!(top: -n).required(false).help("Top fragment file").default_value("20").takes_value(true))
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let top_n = matches.value_of("top").unwrap().parse::<usize>()?;

    println!("{path}:{top_n}");
}
```

## derive

依赖：
```toml
clap = {version = "3.1.10", features = ["derive"] }
```

代码示例：

```rust
// Note: this requires the `derive` feature

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
```
/*!
Disabling parts of the default format.

Before running this example, try setting the `MY_LOG_LEVEL` environment variable to `info`:

```no_run,shell
$ export MY_LOG_LEVEL='info'
```

Also try setting the `MY_LOG_STYLE` environment variable to `never` to disable colors
or `auto` to enable them:

```no_run,shell
$ export MY_LOG_STYLE=never
```

If you want to control the logging output completely, see the `custom_logger` example.
 */

#[macro_use]
extern crate log;

use std::io::Write;
use env_logger::{Builder, Env};

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

fn main() {
    init_logger();

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");
}

/*
output:
[2022-03-03 16:59:26 INFO custom_default_format:59] env_logger initialized.
[2022-03-03 16:59:26 DEBUG custom_default_format:66] some debug log
[2022-03-03 16:59:26 INFO custom_default_format:67] some information log
[2022-03-03 16:59:26 WARN custom_default_format:68] some warning log
[2022-03-03 16:59:26 ERROR custom_default_format:69] some error log
 */
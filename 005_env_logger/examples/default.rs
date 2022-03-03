/*!
Using `env_logger`.

Before running this example, try setting the `MY_LOG_LEVEL` environment variable to `info`:

```no_run,shell
$ export MY_LOG_LEVEL='info'
```

Also try setting the `MY_LOG_STYLE` environment variable to `never` to disable colors
or `auto` to enable them:

```no_run,shell
$ export MY_LOG_STYLE=never
```
*/

#[macro_use]
extern crate log;

use env_logger::Env;

pub fn init_logger() {
    // The `Env` lets us tweak what the environment
    // variables to read are and what the default
    // value is if they're missing
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
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
[2022-03-03T03:49:21Z TRACE default] some trace log
[2022-03-03T03:49:21Z DEBUG default] some debug log
[2022-03-03T03:49:21Z INFO  default] some information log
[2022-03-03T03:49:21Z WARN  default] some warning log
[2022-03-03T03:49:21Z ERROR default] some error log
 */
use time::UtcOffset;
use tracing::{debug, info, Level, trace, warn};
use tracing_subscriber::fmt::time::{LocalTime, OffsetTime};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use time::macros::format_description;


// The filter syntax is a superset of the env_logger syntax.
//
// For example:
//
// Setting RUST_LOG=debug enables all Spans and Events set to the log level DEBUG or higher
// Setting RUST_LOG=my_crate=trace enables Spans and Events in my_crate at all log levels

fn main() {
    // 第一种方式：
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(
    //         std::env::var("RUST_LOG").unwrap_or_else(|_| "tracinglog=debug".to_string()),
    //     ))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    // 第二种,修改时区,
    // let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));
    //
    // //time::UtcOffset::from_hms(8, 0, 0)
    // //let timer = LocalTime::new(UtcOffset::from_hms(8, 0, 0));
    // tracing_subscriber::fmt()
    //     .with_writer(std::io::stdout)
    //     .with_max_level(Level::TRACE)
    //     .with_timer(timer)
    //     .init();

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

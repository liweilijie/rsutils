use tracing::{debug, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// The filter syntax is a superset of the env_logger syntax.
//
// For example:
//
// Setting RUST_LOG=debug enables all Spans and Events set to the log level DEBUG or higher
// Setting RUST_LOG=my_crate=trace enables Spans and Events in my_crate at all log levels

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "tracinglog=debug".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("info.");
    warn!("info.");
    debug!("info.");
}

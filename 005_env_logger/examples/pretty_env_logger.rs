#[macro_use]
extern crate log;
fn main() {
    pretty_env_logger::init();

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");
}


mod client;
mod error;
mod sms;
mod model;

pub use error::Error;
pub use client::Client;
pub use client::Result;
pub use sms::*;
pub use model::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

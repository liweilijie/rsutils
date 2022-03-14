use once_cell::sync::Lazy;
use crate::Keys;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    // let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let secret = std::env::var("JWT_SECRET").unwrap_or("sbsotestrustinfo".into());
    Keys::new(secret.as_bytes())
});


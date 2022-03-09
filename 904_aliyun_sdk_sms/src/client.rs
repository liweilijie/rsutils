use crate::error::Error;
use reqwest::Client as HttpClient;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Client {
    pub(crate) access_key: String,
    pub(crate) secret_key: String,
    pub(crate) http: HttpClient,
}

impl Client {
    pub fn new<S: Into<String>>(access_key: S, secret_key: S) -> Self {
       Client {
           access_key: access_key.into(),
           secret_key: secret_key.into(),
           http: HttpClient::new(),
       }
    }
}
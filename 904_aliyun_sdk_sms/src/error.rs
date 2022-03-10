use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http client error: {0}")]
   Reqwest(#[from] reqwest::Error),

    #[error("internal error occurred: {0}")]
    Internal(String)
}
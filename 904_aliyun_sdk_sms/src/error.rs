use thiserror::Error;
use reqwest::Error as ReqwestError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http client error: {0}")]
   Reqwest(#[from] ReqwestError),

    #[error("internal error occurred: {0}")]
    Internal(String)
}
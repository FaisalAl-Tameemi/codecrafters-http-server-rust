use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid HTTP status code: {0}")]
    InvalidStatusCode(u16),
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Directory not set")]
    DirectoryNotSet,
    #[error("Invalid method")]
    InvalidMethod,
    #[error("Failed to compress payload")]
    FailedToCompressPayload,
}

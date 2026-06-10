use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Something went wrong during an I/O operation.
    #[error("access I/O error: {0}")]
    IoError(io::Error),

    /// Something went wrong during serialization/deserialization.
    #[error("serialization error: {0}")]
    SerError(serde_json::Error),

    /// An unknown error occurred.
    #[error("unknown error: {0}")]
    UnknownError(anyhow::Error),
}

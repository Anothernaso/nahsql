use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    /// Something went wrong during an I/O operation.
    #[error("io error: {0}")]
    IoError(io::Error),

    /// A potential issue with the schema configuration was detected during a database operation.
    #[error("schema error: {0}")]
    SchemaError(String),

    /// Something went wrong during serialization/deserialization.
    #[error("serialization error: {0}")]
    SerError(serde_json::Error),
}

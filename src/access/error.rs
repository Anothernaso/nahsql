use crate::schema;
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

    /// Something does not match the schema.
    #[error("schema error: {0}")]
    SchemaError(schema::Error),

    /// An unknown error occurred.
    #[error("unknown error: {0}")]
    UnknownError(anyhow::Error),
}

use crate::schema;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Something went wrong during an I/O operation.
    #[error("access I/O error: {0}")]
    IoError(#[from] io::Error),

    /// Something went wrong during serialization.
    #[error("serialization error: {0}")]
    SerError(#[from] toml::ser::Error),

    /// Something went wrong during deserialization.
    #[error("deserialization error: {0}")]
    DeError(#[from] toml::de::Error),

    /// Something does not match the schema.
    #[error("schema error: {0}")]
    SchemaError(#[from] schema::Error),

    /// An unknown error occurred.
    #[error("unknown error: {0}")]
    UnknownError(#[from] anyhow::Error),
}

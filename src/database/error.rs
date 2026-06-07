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

    /// A function or API is being used incorrectly.
    #[error("usage error: {0}")]
    UsageError(String),

    /// A requested table does not exist.
    #[error("no such table: {0}")]
    NoSuchTable(String),

    /// A requested field does not exist in the given table.
    #[error("no such field: {field_name}, in table: {table_name}")]
    NoSuchField {
        table_name: String,
        field_name: String,
    },
}

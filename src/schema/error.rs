//! Errors related to the schema of a database

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("no such table: {0}")]
    NoSuchTable(String),

    #[error("no such field in table `{table}`: {field}")]
    NoSuchField { table: String, field: String },
}

//! Errors related to the schema of a database

use crate::variant::VariantType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("no such table: {0}")]
    NoSuchTable(String),

    #[error("no such field in table `{table}`: {field}")]
    NoSuchField { table: String, field: String },

    #[error("type mismatch: expected `{expected}`, but got `{given}`")]
    TypeMismatch {
        expected: VariantType,
        given: VariantType,
    },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

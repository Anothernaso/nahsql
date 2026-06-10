use crate::access;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    /// Something went wrong during an I/O operation.
    #[error("database I/O error: {0}")]
    IoError(io::Error),

    /// Something went wrong while accessing the database.
    #[error("database access error: {0}")]
    AccessError(access::Error),
}

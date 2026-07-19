use thiserror::Error;

use crate::access::Error as AccessError;
use crate::schema::Error as SchemaError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("schema error: {0}")]
    SchemaError(#[from] SchemaError),

    #[error("access error: {0}")]
    AccessError(#[from] AccessError),

    #[error("unknown error: {0}")]
    UnknownError(#[from] anyhow::Error),
}

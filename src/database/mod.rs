mod entry;
mod error;
mod index;
mod manifest;

use crate::schema::Schema;
use error::DbError;
use std::path::{Path, PathBuf};

pub struct Database {
    path: PathBuf,
    schema: Schema,
}

impl Database {
    /// Opens the database at the given `path`
    /// with the given `schema`, creating the database
    /// if it doesn't already exist.
    #[cfg(all(feature = "std"))]
    pub fn open_sync(path: impl Into<PathBuf>, schema: Schema) -> Result<Self, DbError> {
        let path = path.into();

        let db = Self { path, schema };

        Ok(db)
    }

    /// Opens the database at the given `path`
    /// with the given `schema`, creating the database
    /// if it doesn't already exist.
    #[cfg(all(feature = "tokio"))]
    pub async fn open_async(path: impl Into<PathBuf>, schema: Schema) -> Result<Self, DbError> {
        let path = path.into();

        let db = Self { path, schema };

        Ok(db)
    }

    /// Gets the filepath of the database.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Gets the schema of the database.
    pub fn schema(&self) -> &Schema {
        &self.schema
    }
}

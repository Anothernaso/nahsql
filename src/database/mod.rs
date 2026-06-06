pub mod error;
pub mod manifest;
pub mod table;
pub mod validate;

use crate::schema::Schema;
use error::DbError;
use std::path::{Path, PathBuf};
use validate::ImplDbValidate;

const TABLE_DIR: &str = "tables";

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
        db.validate_sync()?;

        Ok(db)
    }

    /// Opens the database at the given `path`
    /// with the given `schema`, creating the database
    /// if it doesn't already exist.
    #[cfg(all(feature = "tokio"))]
    pub async fn open_async(path: impl Into<PathBuf>, schema: Schema) -> Result<Self, DbError> {
        let path = path.into();

        let db = Self { path, schema };
        db.validate_async().await?;

        Ok(db)
    }

    /// Gets the path of the database
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Gets the schema of the database
    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    /// Gets the filepath to the directory where tables are stored.
    pub fn table_dir(&self) -> PathBuf {
        self.path.join(TABLE_DIR)
    }
}

mod contents;
mod error;

pub use contents::*;
pub use error::*;

use crate::{
    access::{read_manifest, write_manifest},
    meta,
    schema::Schema,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

struct DbInner {
    pub path: PathBuf,
    pub schema: Schema,
}

pub struct Database {
    inner: Arc<DbInner>,
}

impl Database {
    /// Opens the database at the given `path`
    /// with the given `schema`, creating the database
    /// if it doesn't already exist.
    pub fn open(path: impl Into<PathBuf>, schema: impl Into<Schema>) -> Result<Self, Error> {
        let path = path.into();
        let schema = schema.into();

        let db = Self {
            inner: Arc::new(DbInner {
                path: path,
                schema: schema,
            }),
        };

        let mut mf = read_manifest(&db).map_err(|e| Error::AccessError(e))?;

        // TODO: verify schema and crate versions

        mf.crate_version = meta::CRATE_VERSION.into();
        mf.schema_version = db.schema().version();
        write_manifest(&db, &mf).map_err(|e| Error::AccessError(e))?;

        Ok(db)
    }

    /// Gets the filepath of the database.
    pub fn path(&self) -> &Path {
        &self.inner.path
    }

    /// Gets the schema of the database.
    pub fn schema(&self) -> &Schema {
        &self.inner.schema
    }
}

impl Clone for Database {
    /// Clones the database's internal smart pointer,
    /// returning a new reference to the same
    /// database to allow for sharing between
    /// threads.
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl AsRef<Database> for Database {
    fn as_ref(&self) -> &Database {
        &self
    }
}

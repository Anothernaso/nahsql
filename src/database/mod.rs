mod contents;
mod error;

pub use contents::*;
pub use error::Error;

use crate::{meta, schema::Schema};
use std::path::{Path, PathBuf};

pub struct Database {
    path: PathBuf,
    schema: Schema,
}

impl Database {
    /// Opens the database at the given `path`
    /// with the given `schema`, creating the database
    /// if it doesn't already exist.
    ///
    /// # Notes
    /// Produces a warning on crate or schema version
    /// mismatch if the `tracing` feature is enabled.
    ///
    pub fn open(path: impl Into<PathBuf>, schema: Schema) -> Result<Self, Error> {
        use crate::access::{read_manifest, write_manifest};

        let path = path.into();
        let db = Self { path, schema };

        let mut mf = read_manifest(&db).map_err(|e| Error::AccessError(e))?;

        // TODO: verify schema and crate versions

        mf.crate_version = meta::CRATE_VERSION.into();
        mf.schema_version = db.schema.version();
        write_manifest(&db, &mf).map_err(|e| Error::AccessError(e))?;

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

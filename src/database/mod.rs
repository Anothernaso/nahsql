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
    #[cfg(all(feature = "sync"))]
    pub fn open_sync(path: impl Into<PathBuf>, schema: Schema) -> Result<Self, Error> {
        use crate::access::{read_manifest_sync, write_manifest_sync};

        let path = path.into();
        let db = Self { path, schema };

        let mut mf = read_manifest_sync(&db).map_err(|e| Error::AccessError(e))?;

        // Print a waring if the `tracing` feature is enable
        // and there are version mismatches.
        #[cfg(all(feature = "tracing"))]
        {
            if mf.crate_version != meta::CRATE_VERSION {
                tracing::warn!(
                    db_ver = mf.crate_version,
                    ver = meta::CRATE_VERSION,
                    "crate version mismatch"
                );
            }

            if mf.schema_version != db.schema.version() {
                tracing::warn!(
                    db_ver = mf.schema_version,
                    ver = db.schema.version(),
                    "schema version mismatch"
                );
            }
        }

        mf.crate_version = meta::CRATE_VERSION.into();
        mf.schema_version = db.schema.version();
        write_manifest_sync(&db, &mf).map_err(|e| Error::AccessError(e))?;

        Ok(db)
    }

    /// Opens the database at the given `path`
    /// with the given `schema`, creating the database
    /// if it doesn't already exist.
    ///
    /// # Notes
    /// Produces a warning on crate or schema version
    /// mismatch if the `tracing` feature is enabled.
    ///
    #[cfg(all(feature = "async"))]
    pub async fn open_async(path: impl Into<PathBuf>, schema: Schema) -> Result<Self, Error> {
        use crate::access::{read_manifest_async, write_manifest_async};

        let path = path.into();
        let db = Self { path, schema };

        let mut mf = read_manifest_async(&db)
            .await
            .map_err(|e| Error::AccessError(e))?;

        // Print a waring if the `tracing` feature is enable
        // and there are version mismatches.
        #[cfg(all(feature = "tracing"))]
        {
            if mf.crate_version != meta::CRATE_VERSION {
                tracing::warn!(
                    db_ver = mf.crate_version,
                    ver = meta::CRATE_VERSION,
                    "crate version mismatch"
                );
            }

            if mf.schema_version != db.schema.version() {
                tracing::warn!(
                    db_ver = mf.schema_version,
                    ver = db.schema.version(),
                    "schema version mismatch"
                );
            }
        }

        mf.crate_version = meta::CRATE_VERSION.into();
        mf.schema_version = db.schema.version();
        write_manifest_async(&db, &mf)
            .await
            .map_err(|e| Error::AccessError(e))?;

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

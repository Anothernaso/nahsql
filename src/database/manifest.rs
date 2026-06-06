use crate::{
    database::{Database, DbError},
    meta,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const MANIFEST_PATH: &str = "manifest.json";

#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[display(
    "DatabaseManifest {{ crate_version: {}, schema_version: {} }}",
    crate_version,
    schema_version
)]
pub struct DbManifest {
    pub crate_version: String,
    pub schema_version: usize,
}

impl DbManifest {
    /// Creates a new database manifest with the `user_version`,
    /// use the version of the `nahsql` crate as the `factory_version`.
    pub fn new(schema_version: usize) -> Self {
        Self {
            crate_version: meta::CRATE_VERSION.into(),
            schema_version,
        }
    }
}

pub trait DbManifestImpl {
    fn mf_path(&self) -> PathBuf;

    #[cfg(all(feature = "std"))]
    fn get_schema_version_sync(&self) -> Result<usize, DbError>;

    #[cfg(all(feature = "tokio"))]
    fn get_schema_version_async(&self) -> impl Future<Output = Result<usize, DbError>> + Send;

    #[cfg(all(feature = "std"))]
    fn get_crate_version_sync(&self) -> Result<String, DbError>;

    #[cfg(all(feature = "tokio"))]
    fn get_crate_version_async(&self) -> impl Future<Output = Result<String, DbError>> + Send;
}

impl DbManifestImpl for Database {
    /// Gets the filepath to the manifest of this database.
    fn mf_path(&self) -> PathBuf {
        self.path.join(MANIFEST_PATH)
    }

    /// Gets the schema version of the database when it was created.
    #[cfg(all(feature = "std"))]
    fn get_schema_version_sync(&self) -> Result<usize, DbError> {
        use std::fs;

        let mf_path = self.mf_path();

        let mf_json = fs::read_to_string(mf_path).map_err(|e| DbError::IoError(e))?;
        let mf = serde_json::from_str::<DbManifest>(&mf_json).map_err(|e| DbError::SerError(e))?;

        Ok(mf.schema_version)
    }

    /// Gets the schema version of the database when it was created.
    #[cfg(all(feature = "tokio"))]
    fn get_schema_version_async(&self) -> impl Future<Output = Result<usize, DbError>> + Send {
        use tokio::fs;

        async move {
            let mf_path = self.mf_path();

            let mf_json = fs::read_to_string(mf_path)
                .await
                .map_err(|e| DbError::IoError(e))?;
            let mf =
                serde_json::from_str::<DbManifest>(&mf_json).map_err(|e| DbError::SerError(e))?;

            Ok(mf.schema_version)
        }
    }

    /// Gets the version of `nahsql` used to create the database.
    #[cfg(all(feature = "std"))]
    fn get_crate_version_sync(&self) -> Result<String, DbError> {
        use std::fs;

        let mf_path = self.mf_path();

        let mf_json = fs::read_to_string(mf_path).map_err(|e| DbError::IoError(e))?;
        let mf = serde_json::from_str::<DbManifest>(&mf_json).map_err(|e| DbError::SerError(e))?;

        Ok(mf.crate_version.trim().into())
    }

    /// Gets the version of `nahsql` used to create the database.
    #[cfg(all(feature = "tokio"))]
    fn get_crate_version_async(&self) -> impl Future<Output = Result<String, DbError>> + Send {
        use tokio::fs;

        async move {
            let mf_path = self.mf_path();

            let mf_json = fs::read_to_string(mf_path)
                .await
                .map_err(|e| DbError::IoError(e))?;
            let mf =
                serde_json::from_str::<DbManifest>(&mf_json).map_err(|e| DbError::SerError(e))?;

            Ok(mf.crate_version.trim().into())
        }
    }
}

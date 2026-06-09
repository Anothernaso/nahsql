use crate::meta::{self, SchemaVersion};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[display(
    "DatabaseManifest {{ crate_version: {}, schema_version: {} }}",
    crate_version,
    schema_version
)]
pub struct DbManifest {
    pub crate_version: String,
    pub schema_version: SchemaVersion,
}

impl DbManifest {
    /// Creates a new database manifest with the `user_version`,
    /// use the version of the `nahsql` crate as the `factory_version`.
    pub fn new(schema_version: SchemaVersion) -> Self {
        Self {
            crate_version: meta::CRATE_VERSION.into(),
            schema_version,
        }
    }
}

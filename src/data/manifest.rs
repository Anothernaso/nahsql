use crate::meta::SchemaVersion;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[display(
    r#"
Database Manifest:
    Crate Version  - {}
    Schema Version - {}
"#,
    crate_version,
    schema_version
)]
pub struct DbManifest {
    crate_version: String,
    schema_version: SchemaVersion,
}

impl DbManifest {
    pub fn new(crate_version: impl Into<String>, schema_version: impl Into<SchemaVersion>) -> Self {
        Self {
            crate_version: crate_version.into(),
            schema_version: schema_version.into(),
        }
    }

    pub fn crate_version(&self) -> &str {
        &self.crate_version
    }

    pub fn crate_version_mut(&mut self) -> &mut String {
        &mut self.crate_version
    }

    pub fn set_crate_version(&mut self, crate_version: impl Into<String>) {
        self.crate_version = crate_version.into();
    }

    pub fn schema_version(&self) -> SchemaVersion {
        self.schema_version
    }

    pub fn set_schema_version(&mut self, schema_version: impl Into<SchemaVersion>) {
        self.schema_version = schema_version.into();
    }
}

impl AsRef<Self> for DbManifest {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl Into<DbManifest> for &DbManifest {
    fn into(self) -> DbManifest {
        self.clone()
    }
}

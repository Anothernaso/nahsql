use super::DbTableEntryImpl;
use crate::{
    database::Database,
    value::{Value, ValueKey},
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

const MANIFEST_FILE: &str = "manifest.json";

#[derive(Debug, Display, Default, Clone, PartialEq, Serialize, Deserialize)]
#[display("DbTableEntryManifest {{ fields: {:?} }}", fields)]
pub struct DbTableEntryManifest {
    /// The structure is `<field_name, field_value>`
    fields: HashMap<String, Value>,
}

pub trait DbTableEntryManifestImpl {
    fn table_entry_manifest_path(
        &self,
        table_name: impl AsRef<str>,
        primary_key: ValueKey,
    ) -> PathBuf;
}

impl DbTableEntryManifestImpl for Database {
    /// Gets the filepath of the manifest of the
    /// given entry inside the given table,
    /// where `table_name` is the name of the table and
    /// `primary_key` is the primary key of the entry.
    ///
    /// # Notes
    ///
    /// The table does not necessarily have to exist in
    /// the filesystem or in the schema for this to work, nor does the entry.
    ///
    fn table_entry_manifest_path(
        &self,
        table_name: impl AsRef<str>,
        primary_key: ValueKey,
    ) -> PathBuf {
        let mut path = self.table_entry_path(table_name, primary_key);
        path.push(MANIFEST_FILE);

        path
    }
}

use crate::{
    database::{Database, table::DbTableImpl},
    value::ValueKey,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, path::PathBuf};

const INDEX_DIR: &str = "indices";

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DbTableIndex {
    /// The structure is `<key_field_value, element_primary_key>`
    elements: HashMap<ValueKey, ValueKey>,
}

impl DbTableIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn elements(&self) -> &HashMap<ValueKey, ValueKey> {
        &self.elements
    }

    pub fn elements_mut(&mut self) -> &mut HashMap<ValueKey, ValueKey> {
        &mut self.elements
    }
}

pub trait DbTableIndexImpl {
    fn table_index_dir(&self, table_name: impl AsRef<str>) -> PathBuf;
    fn table_index_path(&self, table_name: impl AsRef<str>, field_name: impl AsRef<str>)
    -> PathBuf;
}

impl DbTableIndexImpl for Database {
    /// Gets the filepath to the index directory of the given table,
    /// where `table_name` is the name of the table.
    ///
    /// # Notes
    ///
    /// The table does not necessarily have to exist in
    /// the filesystem or in the schema for this to work.
    ///
    fn table_index_dir(&self, table_name: impl AsRef<str>) -> PathBuf {
        let mut path = self.table_path(table_name);
        path.push(INDEX_DIR);

        path
    }

    /// Gets the filepath to the index of the given field in the given table,
    /// where `table_name` is the name of the table and `field_name` is the name of
    /// the field inside the table to get the index of.
    ///
    /// # Notes
    ///
    /// The table does not necessarily have to exist in
    /// the filesystem or in the schema for this to work, nor does the index or the field.
    ///
    fn table_index_path(
        &self,
        table_name: impl AsRef<str>,
        field_name: impl AsRef<str>,
    ) -> PathBuf {
        let mut path = self.table_index_dir(table_name);
        path.push(format!(
            "{}.json",
            hex::encode(Sha256::digest(field_name.as_ref()))
        ));

        path
    }
}

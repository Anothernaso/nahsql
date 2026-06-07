mod index;
pub mod validate;

use crate::{database::Database, value::ValueKey};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

const ENTRY_DIR: &str = "entries";

pub trait DbTableImpl {
    fn table_path(&self, table_name: impl AsRef<str>) -> PathBuf;
    fn table_entry_dir(&self, table_name: impl AsRef<str>) -> PathBuf;
    fn table_entry_path(&self, table_name: impl AsRef<str>, primary_key: ValueKey) -> PathBuf;
}

impl DbTableImpl for Database {
    /// Gets the filepath of the given table.
    ///
    /// # Notes
    ///
    /// The table does not necessarily have to exist in
    /// the filesystem or in the schema for this to work.
    ///
    fn table_path(&self, table_name: impl AsRef<str>) -> PathBuf {
        let mut path = self.table_dir();
        path.push(hex::encode(Sha256::digest(table_name.as_ref())));

        path
    }

    /// Gets the filepath to the entry directory inside the given table,
    /// where `table_name` is the name of the table.
    ///
    /// # Notes
    ///
    /// The table does not necessarily have to exist in
    /// the filesystem or in the schema for this to work.
    ///
    fn table_entry_dir(&self, table_name: impl AsRef<str>) -> PathBuf {
        let mut path = self.table_path(table_name);
        path.push(ENTRY_DIR);

        path
    }

    /// Gets the filepath of the given entry inside the given table,
    /// where `table_name` is the name of the table, and `primary_key` is the
    /// primary key of the table entry.
    ///
    /// # Notes
    ///
    /// The table does not necessarily have to exist in
    /// the filesystem or in the schema for this to work, nor does the entry.
    ///
    fn table_entry_path(&self, table_name: impl AsRef<str>, primary_key: ValueKey) -> PathBuf {
        let mut path = self.table_entry_dir(table_name);
        path.push(hex::encode(Sha256::digest(Vec::<u8>::from(primary_key))));

        path
    }
}

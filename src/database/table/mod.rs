pub mod entry;
pub mod index;

use crate::database::Database;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub trait DbTableImpl {
    fn table_path(&self, table_name: impl AsRef<str>) -> PathBuf;
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
}

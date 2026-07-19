//! API for accessing and modifying
//! the indices of a table in a database.

use super::error::Error;
use crate::{
    data::DbIndex,
    database::{DB_TABLE_DIR, Database, TB_INDEX_DIR},
};
use anyhow::anyhow;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

fn idx_path(db: &Database, table: impl AsRef<str>, field: impl AsRef<str>) -> PathBuf {
    let table = table.as_ref();
    let field = field.as_ref();

    // Append the table directory to the database path.
    let mut path = db.path().join(DB_TABLE_DIR);

    // Append the table hash to the table directory.
    path.push(hex::encode(Sha256::digest(table)));

    // Append the index directory to the table hash.
    path.push(TB_INDEX_DIR);

    // Append the index filename to the index directory.
    //
    // The index filename is gotten by hashing the field name
    // and then appending the `.json` filename extension to the result.
    //
    path.push(format!("{}.json", hex::encode(Sha256::digest(field))));

    path
}

/// Synchronously reads the index of the given field in the
/// given table of the given database.
pub fn read_index(
    db: &Database,
    table: impl AsRef<str>,
    field: impl AsRef<str>,
) -> Result<DbIndex, Error> {
    let table = table.as_ref();
    let field = field.as_ref();

    let path = idx_path(db, table, field);

    let index: DbIndex;

    if fs::exists(&path).map_err(|e| Error::IoError(e))? {
        let file = File::open(path).map_err(|e| Error::IoError(e))?;

        // Use a buffered reader, as index
        // files are expected to be large.
        let buf = BufReader::new(file);

        index = serde_json::from_reader(buf).map_err(|e| Error::SerError(e))?;
    } else {
        index = DbIndex::default();
    }

    Ok(index)
}

/// Writes the given index to the given database.
pub fn write_index(
    db: &Database,
    table: impl AsRef<str>,
    field: impl AsRef<str>,
    index: &DbIndex,
) -> Result<(), Error> {
    let table = table.as_ref();
    let field = field.as_ref();

    let path = idx_path(db, table, field);
    let parent = path.parent().ok_or(Error::UnknownError(anyhow!(
        "database index path has no parent"
    )))?;

    if !fs::exists(parent).map_err(|e| Error::IoError(e))? {
        fs::create_dir_all(parent).map_err(|e| Error::IoError(e))?;
    }

    let file = File::create(path).map_err(|e| Error::IoError(e))?;

    // Use a buffered reader, as index
    // files are expected to be large.
    let buf = BufWriter::new(file);

    serde_json::to_writer(buf, index).map_err(|e| Error::SerError(e))?;

    Ok(())
}

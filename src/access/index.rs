//! API for accessing and modifying
//! the indices of a table in a database.

use super::error::Error;
use crate::{
    data::DbIndex,
    database::{DB_TABLE_DIR, Database, TB_INDEX_DIR},
    schema,
};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

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

/// Reads the index of the given field in the
/// given table of the given database.
#[cfg(all(feature = "sync"))]
pub fn read_index_sync(
    db: &Database,
    table: impl AsRef<str>,
    field: impl AsRef<str>,
) -> Result<DbIndex, Error> {
    use std::{
        fs::{self, File},
        io::BufReader,
    };

    let table = table.as_ref();
    let field = field.as_ref();

    let t = db
        .schema()
        .tables()
        .get(table)
        .ok_or_else(|| Error::SchemaError(schema::Error::NoSuchTable(table.into())))?;
    let _ = t.fields().get(field).ok_or_else(|| {
        Error::SchemaError(schema::Error::NoSuchField {
            table: table.into(),
            field: field.into(),
        })
    })?;

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

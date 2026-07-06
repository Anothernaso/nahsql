//! API for accessing and modifying
//! the indices of a table in a database.

use super::error::Error;
use crate::{
    data::DbIndex,
    database::{DB_TABLE_DIR, Database, TB_INDEX_DIR},
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

/// Synchronously reads the index of the given field in the
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

/// Asynchronously reads the index of the given field in the
/// given table of the given database.
#[cfg(all(feature = "async"))]
pub async fn read_index_async(
    db: &Database,
    table: impl AsRef<str>,
    field: impl AsRef<str>,
) -> Result<DbIndex, Error> {
    use std::{fs::File, io::BufReader};
    use tokio::{fs, task::spawn_blocking};

    let table = table.as_ref();
    let field = field.as_ref();

    let path = idx_path(db, table, field);

    let index: DbIndex;

    if fs::try_exists(&path).await.map_err(|e| Error::IoError(e))? {
        let file = spawn_blocking(|| -> Result<File, Error> {
            Ok(File::open(path).map_err(|e| Error::IoError(e))?)
        })
        .await
        .expect("could not join blocking task")?;

        // Use a buffered reader, as index
        // files are expected to be large.
        let buf = spawn_blocking(|| BufReader::new(file))
            .await
            .expect("could not join blocking task");

        index = spawn_blocking(|| -> Result<DbIndex, Error> {
            Ok(serde_json::from_reader(buf).map_err(|e| Error::SerError(e))?)
        })
        .await
        .expect("could not join blocking task")?;
    } else {
        index = DbIndex::default();
    }

    Ok(index)
}

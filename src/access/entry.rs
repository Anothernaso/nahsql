use crate::{
    access::Error,
    data::DbEntry,
    database::{DB_TABLE_DIR, Database, ET_ENTRY_FILE, TB_ENTRY_DIR},
    value::ValueKey,
};
use anyhow::anyhow;
use hex;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

fn entry_path(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    primary_key: impl Into<ValueKey>,
) -> PathBuf {
    let db = db.as_ref();
    let table = table.as_ref();
    let primary_key = primary_key.into();

    // Append the table directory to the database path.
    let mut path = db.path().join(DB_TABLE_DIR);

    // Append the table hash to the table directory.
    path.push(hex::encode(Sha256::digest(table)));

    // Append the entry directory to the table path.
    path.push(TB_ENTRY_DIR);

    // Append the primary key to the entry directory.
    path.push(hex::encode(Sha256::digest(
        <ValueKey as Into<Vec<u8>>>::into(primary_key),
    )));

    // Append the entry file to the entry path.
    path.push(ET_ENTRY_FILE);

    path
}

pub fn read_entry(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    primary_key: impl Into<ValueKey>,
) -> Result<DbEntry, Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let primary_key = primary_key.into();

    let path = entry_path(db, table, primary_key);

    let entry: DbEntry;

    if fs::exists(&path).map_err(|e| Error::IoError(e))? {
        let file = File::open(path).map_err(|e| Error::IoError(e))?;

        let buf = BufReader::new(file);

        entry = serde_json::from_reader(buf).map_err(|e| Error::SerError(e))?;
    } else {
        entry = DbEntry::default();
    }

    Ok(entry)
}

pub fn write_entry(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    primary_key: impl Into<ValueKey>,
    entry: impl AsRef<DbEntry>,
) -> Result<(), Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let primary_key = primary_key.into();
    let entry = entry.as_ref();

    let path = entry_path(db, table, primary_key);
    let parent = path.parent().ok_or(Error::UnknownError(anyhow!(
        "database entry path has no parent"
    )))?;

    if !fs::exists(parent).map_err(|e| Error::IoError(e))? {
        fs::create_dir_all(parent).map_err(|e| Error::IoError(e))?;
    }

    let file = File::create(path).map_err(|e| Error::IoError(e))?;
    let buf = BufWriter::new(file);

    serde_json::to_writer(buf, entry).map_err(|e| Error::SerError(e))?;

    Ok(())
}

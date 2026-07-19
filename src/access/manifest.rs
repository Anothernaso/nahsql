//! API for accessing and modifying
//! the manifest file of a database.

use super::error::Error;
use crate::{
    data::DbManifest,
    database::{DB_MANIF_FILE, Database},
};
use anyhow::anyhow;
use std::{fs, path::PathBuf};

fn mf_path(db: &Database) -> PathBuf {
    db.path().join(DB_MANIF_FILE)
}

/// Reads the manifest file of the database synchronously.
///
/// Returns an error if anything goes wrong
///
pub fn read_manifest(db: impl AsRef<Database>) -> Result<DbManifest, Error> {
    let db = db.as_ref();
    let mf_path = mf_path(db);

    let mf: DbManifest;
    if fs::exists(&mf_path)? {
        let mf_str = fs::read_to_string(mf_path)?;
        mf = serde_json::from_str(&mf_str)?;
    } else {
        mf = DbManifest::new(db.schema().version());
    }

    Ok(mf)
}

/// Writes the given manifest to the given database synchronously.
///
/// Returns an error if anything goes wrong
///
pub fn write_manifest(db: impl AsRef<Database>, mf: impl AsRef<DbManifest>) -> Result<(), Error> {
    let db = db.as_ref();
    let mf = mf.as_ref();

    let path = mf_path(db);
    let parent = path
        .parent()
        .ok_or(anyhow!("database manifest path has no parent"))?;

    if !fs::exists(&parent)? {
        fs::create_dir_all(parent)?;
    }

    let mf_str = serde_json::to_string(mf)?;
    fs::write(path, &mf_str)?;

    Ok(())
}

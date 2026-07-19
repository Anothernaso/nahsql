//! API for accessing and modifying
//! the manifest file of a database.

use super::error::Error;
use crate::{
    data::DbManifest,
    database::{DB_MANIF_FILE, Database},
};
use anyhow::anyhow;
use std::path::PathBuf;

fn mf_path(db: &Database) -> PathBuf {
    let mut path = db.path().to_owned();
    path.push(DB_MANIF_FILE);

    path
}

/// Reads the manifest file of the database synchronously.
///
/// Returns an error if anything goes wrong
///
pub fn read_manifest(db: &Database) -> Result<DbManifest, Error> {
    use std::fs;

    let mf_path = mf_path(db);

    let mf: DbManifest;
    if fs::exists(&mf_path).map_err(|e| Error::IoError(e))? {
        let mf_str = fs::read_to_string(mf_path).map_err(|e| Error::IoError(e))?;
        mf = serde_json::from_str(&mf_str).map_err(|e| Error::SerError(e))?;
    } else {
        mf = DbManifest::new(db.schema().version());
    }

    Ok(mf)
}

/// Writes the given manifest to the given database synchronously.
///
/// Returns an error if anything goes wrong
///
pub fn write_manifest(db: &Database, mf: &DbManifest) -> Result<(), Error> {
    use std::fs;

    let path = mf_path(db);
    let parent = path.parent().ok_or(Error::UnknownError(anyhow!(
        "database manifest path has no parent"
    )))?;

    if !fs::exists(&parent).map_err(|e| Error::IoError(e))? {
        fs::create_dir_all(parent).map_err(|e| Error::IoError(e))?;
    }

    let mf_str = serde_json::to_string_pretty(mf).map_err(|e| Error::SerError(e))?;
    fs::write(path, &mf_str).map_err(|e| Error::IoError(e))?;

    Ok(())
}

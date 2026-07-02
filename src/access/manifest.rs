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
#[cfg(all(feature = "sync"))]
pub fn read_manifest_sync(db: &Database) -> Result<DbManifest, Error> {
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

/// Reads the manifest file of the given database asynchronously.
///
/// Returns an error if anything goes wrong
///
#[cfg(all(feature = "async"))]
pub async fn read_manifest_async(db: &Database) -> Result<DbManifest, Error> {
    use tokio::fs;

    let mf_path = mf_path(db);

    let mf: DbManifest;
    if fs::try_exists(&mf_path)
        .await
        .map_err(|e| Error::IoError(e))?
    {
        let mf_str = fs::read_to_string(mf_path)
            .await
            .map_err(|e| Error::IoError(e))?;
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
#[cfg(all(feature = "sync"))]
pub fn write_manifest_sync(db: &Database, mf: &DbManifest) -> Result<(), Error> {
    use std::fs;

    let mf_path = mf_path(db);
    let mf_dir = mf_path.parent().ok_or(Error::UnknownError(anyhow!(
        "database manifest path has no parent"
    )))?;

    if !fs::exists(&mf_dir).map_err(|e| Error::IoError(e))? {
        fs::create_dir_all(mf_dir).map_err(|e| Error::IoError(e))?;
    }

    let mf_str = serde_json::to_string_pretty(mf).map_err(|e| Error::SerError(e))?;
    fs::write(mf_path, &mf_str).map_err(|e| Error::IoError(e))?;

    Ok(())
}

/// Writes the given manifest to the given database asynchronously.
///
/// Returns an error if anything goes wrong
///
#[cfg(all(feature = "async"))]
pub async fn write_manifest_async(db: &Database, mf: &DbManifest) -> Result<(), Error> {
    use tokio::fs;

    let mf_path = mf_path(db);
    let mf_dir = mf_path.parent().ok_or(Error::UnknownError(anyhow!(
        "database manifest path has no parent"
    )))?;

    if !fs::try_exists(&mf_dir)
        .await
        .map_err(|e| Error::IoError(e))?
    {
        fs::create_dir_all(mf_dir)
            .await
            .map_err(|e| Error::IoError(e))?;
    }

    let mf_str = serde_json::to_string_pretty(mf).map_err(|e| Error::SerError(e))?;
    fs::write(mf_path, &mf_str)
        .await
        .map_err(|e| Error::IoError(e))?;

    Ok(())
}

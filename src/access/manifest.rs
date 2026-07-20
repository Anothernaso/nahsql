//! API for accessing and modifying
//! the manifest file of a database.

use super::error::Error;
use crate::{data::DbManifest, database::Database, meta, path};
use std::fs;

/// Reads the manifest file of the database synchronously.
///
/// Returns an error if anything goes wrong
///
pub fn read_manifest(db: impl AsRef<Database>) -> Result<DbManifest, Error> {
    let db = db.as_ref();

    let path = path::db_inst_manif_file_path(db.path());

    let mf: DbManifest;
    if fs::exists(&path)? {
        let mf_str = fs::read_to_string(path)?;
        mf = serde_json::from_str(&mf_str)?;
    } else {
        mf = DbManifest::new(meta::CRATE_VERSION, db.schema().version());
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

    let parent = db.path();
    let path = path::db_inst_manif_file_path(parent);

    if !fs::exists(parent)? {
        fs::create_dir_all(parent)?;
    }

    let mf_str = serde_json::to_string(mf)?;
    fs::write(path, &mf_str)?;

    Ok(())
}

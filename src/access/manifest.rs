//! API for accessing and modifying
//! the manifest file of a database.

use super::error::Error;
use crate::{data::DbManifest, database::Database, meta, path};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
};

/// Reads the manifest file of the database synchronously.
///
/// Returns an error if anything goes wrong
///
pub fn read_manifest(db: impl AsRef<Database>) -> Result<DbManifest, Error> {
    let db = db.as_ref();

    let path = path::db_inst_manif_file_path(db.path());

    let mf: DbManifest;
    if fs::exists(&path)? {
        let file = File::open(path)?;
        let buf = BufReader::new(file);

        mf = serde_json::from_reader(buf)?;
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

    let file = File::create(path)?;
    let buf = BufWriter::new(file);

    serde_json::to_writer_pretty(buf, mf)?;

    Ok(())
}

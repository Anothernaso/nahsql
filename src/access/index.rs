//! API for accessing and modifying
//! the indices of a table in a database.

use super::error::Error;
use crate::{
    data::TbIndex,
    database::Database,
    path::{self},
};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
};

/// Synchronously reads the index of the given field in the
/// given table of the given database.
pub fn read_index(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    field: impl AsRef<str>,
) -> Result<TbIndex, Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let field = field.as_ref();

    let path = path::index_inst_file_path(
        path::index_dir_path(path::table_inst_dir_path(
            path::table_dir_path(db.path()),
            table,
        )),
        field,
    );

    let index: TbIndex;

    if fs::exists(&path)? {
        let file = File::open(path)?;

        // Use a buffered reader, as index
        // files are expected to be large.
        let buf = BufReader::new(file);

        index = serde_json::from_reader(buf)?;
    } else {
        index = TbIndex::default();
    }

    Ok(index)
}

/// Writes the given index to the given database.
pub fn write_index(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    field: impl AsRef<str>,
    index: impl AsRef<TbIndex>,
) -> Result<(), Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let field = field.as_ref();
    let index = index.as_ref();

    let parent = path::index_dir_path(path::table_inst_dir_path(
        path::table_dir_path(db.path()),
        table,
    ));
    let path = path::index_inst_file_path(&parent, field);

    if !fs::exists(&parent)? {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;

    // Use a buffered reader, as index
    // files are expected to be large.
    let buf = BufWriter::new(file);

    serde_json::to_writer(buf, index)?;

    Ok(())
}

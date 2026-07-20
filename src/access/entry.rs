use crate::{
    access::Error,
    data::TbEntry,
    database::Database,
    path::{self},
    value::ValueKey,
};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
};

pub fn read_entry(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    primary_key: impl Into<ValueKey>,
) -> Result<TbEntry, Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let primary_key = primary_key.into();

    let path = path::entry_inst_file_path(path::entry_inst_dir_path(
        path::entry_dir_path(path::table_inst_dir_path(
            path::table_dir_path(db.path()),
            table,
        )),
        primary_key,
    ));

    let entry: TbEntry;

    if fs::exists(&path)? {
        let file = File::open(path)?;

        let buf = BufReader::new(file);

        entry = serde_json::from_reader(buf)?;
    } else {
        entry = TbEntry::default();
    }

    Ok(entry)
}

pub fn write_entry(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    primary_key: impl Into<ValueKey>,
    entry: impl AsRef<TbEntry>,
) -> Result<(), Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let primary_key = primary_key.into();
    let entry = entry.as_ref();

    let parent = path::entry_inst_dir_path(
        path::entry_dir_path(path::table_inst_dir_path(
            path::table_dir_path(db.path()),
            table,
        )),
        primary_key,
    );
    let path = path::entry_inst_file_path(&parent);

    if !fs::exists(&parent)? {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    let buf = BufWriter::new(file);

    serde_json::to_writer(buf, entry)?;

    Ok(())
}

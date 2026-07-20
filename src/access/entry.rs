use crate::{
    access::Error,
    data::TbEntry,
    database::Database,
    path::{self},
    value::ValueKey,
};
use std::fs;

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
        let entry_toml = fs::read_to_string(&path)?;
        entry = toml::from_str(&entry_toml)?;
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

    let entry_toml = toml::to_string_pretty(entry)?;
    fs::write(path, entry_toml)?;

    Ok(())
}

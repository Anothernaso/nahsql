use crate::path::{self};
use crate::{access::Error, data::DbTable, database::Database};
use std::fs;

pub fn read_table_mf(db: impl AsRef<Database>, table: impl AsRef<str>) -> Result<DbTable, Error> {
    let db = db.as_ref();
    let table = table.as_ref();

    let path = path::table_inst_file_path(path::table_inst_dir_path(
        path::table_dir_path(db.path()),
        table,
    ));

    let mf: DbTable;

    if fs::exists(&path)? {
        let mf_toml = fs::read_to_string(&path)?;
        mf = toml::from_str(&mf_toml)?;
    } else {
        mf = DbTable::default();
    }

    Ok(mf)
}

pub fn write_table_mf(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    mf: impl AsRef<DbTable>,
) -> Result<(), Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let mf = mf.as_ref();

    let parent = path::table_inst_dir_path(path::table_dir_path(db.path()), table);
    let path = path::table_inst_file_path(&parent);

    if !fs::exists(&parent)? {
        fs::create_dir_all(parent)?;
    }

    let mf_str = toml::to_string_pretty(mf)?;
    fs::write(path, mf_str)?;

    Ok(())
}

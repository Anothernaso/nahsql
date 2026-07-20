use crate::path::{self};
use crate::{access::Error, data::DbTable, database::Database};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
};

pub fn read_table_mf(db: impl AsRef<Database>, table: impl AsRef<str>) -> Result<DbTable, Error> {
    let db = db.as_ref();
    let table = table.as_ref();

    let path = path::table_inst_file_path(path::table_inst_dir_path(
        path::table_dir_path(db.path()),
        table,
    ));

    let mf: DbTable;

    if fs::exists(&path)? {
        let file = File::open(path)?;

        let buf = BufReader::new(file);

        mf = serde_json::from_reader(buf)?;
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

    let file = File::create(path)?;

    let buf = BufWriter::new(file);

    serde_json::to_writer(buf, mf)?;

    Ok(())
}

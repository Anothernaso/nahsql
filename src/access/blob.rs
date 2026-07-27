use crate::{access::Error, database::Database, path, variant::KeyVariant};
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Write},
};

pub fn read_blob(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    primary_key: impl Into<KeyVariant>,
    field: impl AsRef<str>,
) -> Result<impl Read, Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let primary_key = primary_key.into();
    let field = field.as_ref();

    let path = path::blob_inst_file_name(
        path::blob_dir_path(path::entry_inst_dir_path(
            path::entry_dir_path(path::table_inst_dir_path(
                path::table_dir_path(db.path()),
                table,
            )),
            primary_key,
        )),
        field,
    );

    let file: Box<dyn Read>;
    if fs::exists(&path)? {
        file = Box::new(File::open(path)?);
    } else {
        file = Box::new(io::empty());
    }

    let buf = BufReader::new(file);

    Ok(buf)
}

pub fn write_blob(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    primary_key: impl Into<KeyVariant>,
    field: impl AsRef<str>,
) -> Result<impl Write, Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let primary_key = primary_key.into();
    let field = field.as_ref();

    let parent = path::blob_dir_path(path::entry_inst_dir_path(
        path::entry_dir_path(path::table_inst_dir_path(
            path::table_dir_path(db.path()),
            table,
        )),
        primary_key,
    ));

    let path = path::blob_inst_file_name(parent.clone(), field);

    if !fs::exists(&parent)? {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    let write = BufWriter::new(file);

    Ok(write)
}

pub fn remove_blob(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    primary_key: impl Into<KeyVariant>,
    field: impl AsRef<str>,
) -> Result<(), Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let primary_key = primary_key.into();
    let field = field.as_ref();

    let path = path::blob_inst_file_name(
        path::blob_dir_path(path::entry_inst_dir_path(
            path::entry_dir_path(path::table_inst_dir_path(
                path::table_dir_path(db.path()),
                table,
            )),
            primary_key,
        )),
        field,
    );

    if fs::exists(&path)? {
        fs::remove_file(path)?;
    }

    Ok(())
}

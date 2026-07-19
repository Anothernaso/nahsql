use crate::{
    access::Error,
    data::TbManifest,
    database::{DB_TABLE_DIR, Database, TB_MANIF_FILE},
};
use anyhow::anyhow;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

fn table_mf_path(db: impl AsRef<Database>, table: impl AsRef<str>) -> PathBuf {
    let db = db.as_ref();
    let table = table.as_ref();

    // Append the table directory to the database path
    let mut path = db.path().join(DB_TABLE_DIR);

    // Append the table name to the table directory
    path.push(hex::encode(Sha256::digest(table)));

    // Append the manifest file to the table path
    path.push(TB_MANIF_FILE);

    path
}

pub fn read_table_mf(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
) -> Result<TbManifest, Error> {
    let db = db.as_ref();
    let table = table.as_ref();

    let path = table_mf_path(db, table);

    let mf: TbManifest;

    if fs::exists(&path)? {
        let file = File::open(path)?;

        let buf = BufReader::new(file);

        mf = serde_json::from_reader(buf)?;
    } else {
        mf = TbManifest::default();
    }

    Ok(mf)
}

pub fn write_table_mf(
    db: impl AsRef<Database>,
    table: impl AsRef<str>,
    mf: impl AsRef<TbManifest>,
) -> Result<(), Error> {
    let db = db.as_ref();
    let table = table.as_ref();
    let mf = mf.as_ref();

    let path = table_mf_path(db, table);
    let parent = path
        .parent()
        .ok_or(anyhow!("table manifest path has no parent"))?;

    if !fs::exists(parent)? {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;

    let buf = BufWriter::new(file);

    serde_json::to_writer(buf, mf)?;

    Ok(())
}

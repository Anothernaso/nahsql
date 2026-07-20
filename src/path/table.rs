use crate::database::{DB_TABLE_DIR_NAME, TB_TABLE_FILE_NAME};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub fn table_dir_path(db_inst_dir_path: impl Into<PathBuf>) -> PathBuf {
    let mut db_inst_dir_path = db_inst_dir_path.into();

    db_inst_dir_path.push(DB_TABLE_DIR_NAME);

    db_inst_dir_path
}

pub fn table_inst_dir_path(
    table_dir_path: impl Into<PathBuf>,
    table_name: impl AsRef<str>,
) -> PathBuf {
    let mut table_dir_path = table_dir_path.into();
    let table_name = table_name.as_ref();

    table_dir_path.push(hex::encode(Sha256::digest(table_name)));

    table_dir_path
}

pub fn table_inst_file_path(table_inst_dir_path: impl Into<PathBuf>) -> PathBuf {
    let mut table_inst_dir_path = table_inst_dir_path.into();

    table_inst_dir_path.push(TB_TABLE_FILE_NAME);

    table_inst_dir_path
}

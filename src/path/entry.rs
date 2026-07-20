use crate::{
    database::{ET_ENTRY_FILE_NAME, TB_ENTRY_DIR_NAME},
    value::ValueKey,
};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub fn entry_dir_path(table_inst_dir_path: impl Into<PathBuf>) -> PathBuf {
    let mut table_inst_dir_path = table_inst_dir_path.into();

    table_inst_dir_path.push(TB_ENTRY_DIR_NAME);

    table_inst_dir_path
}

pub fn entry_inst_dir_path(
    entry_dir_path: impl Into<PathBuf>,
    primary_key_value: impl Into<ValueKey>,
) -> PathBuf {
    let mut entry_dir_path = entry_dir_path.into();
    let primary_key_value = primary_key_value.into();

    entry_dir_path.push(hex::encode(Sha256::digest(
        <ValueKey as Into<Vec<u8>>>::into(primary_key_value),
    )));

    entry_dir_path
}

pub fn entry_inst_file_path(entry_inst_dir_path: impl Into<PathBuf>) -> PathBuf {
    let mut entry_inst_dir_path = entry_inst_dir_path.into();

    entry_inst_dir_path.push(ET_ENTRY_FILE_NAME);

    entry_inst_dir_path
}

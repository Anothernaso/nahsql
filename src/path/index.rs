use crate::database::TB_INDEX_DIR_NAME;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub fn index_dir_path(table_inst_dir_path: impl Into<PathBuf>) -> PathBuf {
    let mut table_inst_dir_path = table_inst_dir_path.into();

    table_inst_dir_path.push(TB_INDEX_DIR_NAME);

    table_inst_dir_path
}

pub fn index_inst_file_path(
    index_dir_path: impl Into<PathBuf>,
    field_name: impl AsRef<str>,
) -> PathBuf {
    let mut index_dir_path = index_dir_path.into();
    let field_name = field_name.as_ref();

    index_dir_path.push(format!("{}.json", hex::encode(Sha256::digest(field_name))));

    index_dir_path
}

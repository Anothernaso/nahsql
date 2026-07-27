use crate::database::{BL_BLOB_FILE_NAME_EXT, ET_BLOB_DIR_NAME};
use std::path::PathBuf;

pub fn blob_dir_path(entry_inst_dir_path: impl Into<PathBuf>) -> PathBuf {
    let mut entry_inst_dir_path = entry_inst_dir_path.into();

    entry_inst_dir_path.push(ET_BLOB_DIR_NAME);

    entry_inst_dir_path
}

pub fn blob_inst_file_name(
    blob_dir_path: impl Into<PathBuf>,
    field_name: impl AsRef<str>,
) -> PathBuf {
    let mut blob_dir_path = blob_dir_path.into();
    let field_name = field_name.as_ref();

    blob_dir_path.push(format!("{}.{}", field_name, BL_BLOB_FILE_NAME_EXT));

    blob_dir_path
}

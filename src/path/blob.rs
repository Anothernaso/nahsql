use crate::database::ET_BLOB_DIR_NAME;
use std::path::PathBuf;

pub fn blob_dir_path(entry_inst_dir_path: impl Into<PathBuf>) -> PathBuf {
    let mut entry_inst_dir_path = entry_inst_dir_path.into();

    entry_inst_dir_path.push(ET_BLOB_DIR_NAME);

    entry_inst_dir_path
}

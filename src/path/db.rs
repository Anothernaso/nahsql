use std::path::PathBuf;

use crate::database::DB_MANIF_FILE_NAME;

pub fn db_inst_manif_file_path(db_inst_dir_path: impl Into<PathBuf>) -> PathBuf {
    let mut db_inst_dir_path = db_inst_dir_path.into();

    db_inst_dir_path.push(DB_MANIF_FILE_NAME);

    db_inst_dir_path
}

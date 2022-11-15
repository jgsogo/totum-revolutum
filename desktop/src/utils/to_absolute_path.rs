use std::{
    env,
    path::{Path, PathBuf},
};

use path_clean::PathClean;

pub fn to_absolute_path(path: &Path) -> PathBuf {
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .expect("Cannot return current dir")
            .join(path)
    };
    path.clean()
}

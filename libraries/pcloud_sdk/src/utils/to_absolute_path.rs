use camino::{Utf8Path, Utf8PathBuf};

use crate::utils::current_path;

pub fn to_absolute_path(path: &Utf8Path) -> Utf8PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        current_path().join(path)
    }
}

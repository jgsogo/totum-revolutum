use std::env;

use camino::{Utf8Path, Utf8PathBuf};
use path_clean::PathClean;

pub fn to_absolute_path(path: &Utf8Path) -> Utf8PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        let current_dir = Utf8PathBuf::from_path_buf(env::current_dir().expect("Cannot return current dir")).unwrap();
        current_dir.join(path)
    }
}

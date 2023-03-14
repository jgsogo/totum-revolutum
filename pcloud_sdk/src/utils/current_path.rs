use std::env;

use camino::{Utf8Path, Utf8PathBuf};

pub fn current_path() -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(env::current_dir().expect("Cannot return current dir")).unwrap()
}

use std::env;

use camino::Utf8PathBuf;

/// Returns the current path as a [`Utf8PathBuf`], or panics
pub fn current_path() -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(env::current_dir().expect("Cannot return current dir")).unwrap()
}

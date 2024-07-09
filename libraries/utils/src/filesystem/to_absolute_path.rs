use camino::{Utf8Path, Utf8PathBuf};

use crate::filesystem::current_path;

/// Returns path as absolute. If it's a relative path it will join it to [`current_path()`]
pub fn to_absolute_path<P: AsRef<Utf8Path>>(path: P) -> Utf8PathBuf {
    if path.as_ref().is_absolute() {
        path.as_ref().to_path_buf()
    } else {
        current_path().join(path)
    }
}

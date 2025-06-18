use ignore::Match;

use super::{DirectoryPath, FilePath, Result};
use camino::{Utf8Path, Utf8PathBuf};

/// A wrapper over [`ignore_files::IgnoreFilter`] that stores the root path
/// of the underlying filesystem it applies to.
///
/// The main purpose is to hide `root_path` from the user, this is required for
/// some implementations of [`Filesystem`] where the exposed filesystem is just
/// a subdirectory (or remote) of some underlying filesystem
pub struct IgnoreFilter {
    ignore_filter: ignore_files::IgnoreFilter,
    root_path: Utf8PathBuf,
}

impl IgnoreFilter {
    pub(crate) fn new(root_path: &Utf8Path) -> Self {
        Self {
            root_path: root_path.into(),
            ignore_filter: ignore_files::IgnoreFilter::empty(root_path),
        }
    }

    pub fn visit_directory(&self, dir: &DirectoryPath) -> bool {
        let fullpath = self.root_path.join(dir);
        self.ignore_filter.check_dir(fullpath.as_std_path())
    }

    pub fn visit_file(&self, filepath: &FilePath) -> bool {
        let fullpath = self.root_path.join(filepath);
        match self.ignore_filter.match_path(fullpath.as_std_path(), false) {
            Match::None => true,
            Match::Ignore(glob) => {
                if glob.from().map_or(true, |f| fullpath.strip_prefix(f).is_ok()) {
                    // Positive match (fail)
                    false
                } else {
                    // Positive match, but not in scope (pass)
                    true
                }
            }
            Match::Whitelist(_) => true,
        }
    }

    /// Add ignore patterns. Use `applies_in` argument to indicate that the patterns should only
    /// apply to some subdirectories.
    pub fn add_globs(&mut self, globs: &[&str], applies_in: Option<&DirectoryPath>) -> Result<()> {
        let applies_in = applies_in.map(|v| self.root_path.join(v).into_std_path_buf());
        Ok(self.ignore_filter.add_globs(globs, applies_in.as_ref())?)
    }
}

use ignore::Match;
use ignore_files::IgnoreFilter;

use crate::{DirectoryPath, FilePath};

pub trait IgnoreFilterT {
    fn visit_directory(&self, dir: &DirectoryPath) -> bool;
    fn visit_file(&self, filepath: &FilePath) -> bool;
}

impl IgnoreFilterT for IgnoreFilter {
    fn visit_directory(&self, dir: &DirectoryPath) -> bool {
        self.check_dir(dir.as_std_path())
    }

    fn visit_file(&self, filepath: &FilePath) -> bool {
        match self.match_path(filepath.as_std_path(), false) {
            Match::None => true,
            Match::Ignore(glob) => {
                if glob
                    .from()
                    .map_or(true, |f| filepath.as_std_path().strip_prefix(f).is_ok())
                {
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
}

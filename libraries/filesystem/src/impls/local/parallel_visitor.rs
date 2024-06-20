use std::str::FromStr;

use camino::{Utf8Path, Utf8PathBuf};
use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};
use tracing::error;

use crate::{DirectoryPathBuf, FileMetadata, FilePathBuf, FilenameBuf};

use super::file_metadata::LocalMetadata;

struct Visitor {
    tx: flume::Sender<Box<dyn FileMetadata>>,
    base_path: Utf8PathBuf,
}

impl Visitor {
    pub fn new(tx: flume::Sender<Box<dyn FileMetadata>>, base_path: Utf8PathBuf) -> Visitor {
        Visitor { tx, base_path }
    }
}

impl ParallelVisitor for Visitor {
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> WalkState {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            // FIXME: Is this the full path with or without root?
            // TODO: Handle this errors
            let entry_path = Utf8Path::from_path(entry.path()).unwrap();
            let relative_path = entry_path.strip_prefix(&self.base_path).unwrap();
            let dirname = DirectoryPathBuf::try_from(
                relative_path
                    .parent()
                    .unwrap_or(DirectoryPathBuf::root().as_utf8_path()),
            )
            .unwrap();
            let filepath = FilePathBuf::new(
                dirname,
                FilenameBuf::from_str(relative_path.file_name().unwrap()).unwrap(),
            );
            let data = match LocalMetadata::from_filesystem(entry_path, &filepath) {
                Ok(metadata) => metadata,
                Err(e) => {
                    error!("Error building LocalMetadata from path: {e}. Quit visiting.");
                    return WalkState::Quit;
                }
            };

            if let Err(e) = self.tx.send(Box::new(data)) {
                error!("Error sending direntry metadata: {e}. Quit visiting.");
                return WalkState::Quit;
            }
        }
        WalkState::Continue
    }
}

pub(crate) struct VisitorBuilder {
    tx: flume::Sender<Box<dyn FileMetadata>>,
    base_path: Utf8PathBuf,
}

impl VisitorBuilder {
    pub fn new(tx: flume::Sender<Box<dyn FileMetadata>>, base_path: Utf8PathBuf) -> VisitorBuilder {
        VisitorBuilder { tx, base_path }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for VisitorBuilder {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        Box::new(Visitor::new(self.tx.clone(), self.base_path.clone()))
    }
}

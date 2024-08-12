use camino::{Utf8Path, Utf8PathBuf};
use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};
use tracing::error;

use crate::{DirectoryPathBuf, Error, FileMetadata, FilePathBuf, FilenameBuf};

struct Visitor {
    tx: flume::Sender<FileMetadata>,
    base_path: Utf8PathBuf,
}

impl Visitor {
    pub fn new(tx: flume::Sender<FileMetadata>, base_path: Utf8PathBuf) -> Visitor {
        Visitor { tx, base_path }
    }
}

impl ParallelVisitor for Visitor {
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> WalkState {
        let action = || -> crate::Result<WalkState> {
            let entry = entry.map_err(|e| Error::Other(format!("Error in ignore file: {}", e)))?;
            match entry.file_type() {
                None => Ok(WalkState::Continue),
                Some(file_type) => {
                    if file_type.is_file() {
                        let filepath = {
                            let as_utf8_path = Utf8Path::from_path(entry.path())
                                .ok_or(Error::InvalidPath)?
                                .strip_prefix(&self.base_path)
                                .map_err(|_| Error::InvalidPath)?;
                            let directory = DirectoryPathBuf::try_from(
                                as_utf8_path.parent().unwrap_or(DirectoryPathBuf::root().as_ref()),
                            )?;
                            let filename = FilenameBuf::fix_and_create(as_utf8_path.file_name().unwrap())?;
                            FilePathBuf::new(directory, filename)
                        };

                        let size = std::fs::metadata(entry.path()).map_err(Error::IoError)?.len();
                        let hash = sha256::try_digest(entry.path())?;
                        let data = FileMetadata {
                            path: filepath,
                            hash,
                            size,
                        };
                        self.tx
                            .send(data)
                            .map_err(|e| Error::Other(format!("Error sending data through flume channel: {}", e)))?;
                        Ok(WalkState::Continue)
                    } else {
                        Ok(WalkState::Continue)
                    }
                }
            }
        };

        action().unwrap_or_else(|e| {
            error!("Error visiting files: {e}");
            WalkState::Quit
        })
    }
}

pub(crate) struct VisitorBuilder {
    tx: flume::Sender<FileMetadata>,
    base_path: Utf8PathBuf,
}

impl VisitorBuilder {
    pub fn new(tx: flume::Sender<FileMetadata>, base_path: Utf8PathBuf) -> VisitorBuilder {
        VisitorBuilder { tx, base_path }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for VisitorBuilder {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        Box::new(Visitor::new(self.tx.clone(), self.base_path.clone()))
    }
}

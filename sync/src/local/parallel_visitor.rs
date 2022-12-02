use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};
use tracing::error;

use crate::diff::Filesystem;
use crate::local::FilesystemLocal;

pub struct Visitor<'s> {
    diff: &'s FilesystemLocal,
}

impl<'s> Visitor<'s> {
    pub fn new(diff: &'s FilesystemLocal) -> Visitor<'s> {
        Visitor { diff }
    }
}

impl<'s> ParallelVisitor for Visitor<'s> {
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> WalkState {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            if let Err(e) = self.diff.file_found(entry) {
                error!("Error sending direntry: {e}. Quit visiting.");
                return WalkState::Quit;
            }
        }
        WalkState::Continue
    }
}

pub struct VisitorBuilder<'s> {
    diff: &'s FilesystemLocal,
}

impl<'s> VisitorBuilder<'s> {
    pub fn new(diff: &FilesystemLocal) -> VisitorBuilder {
        VisitorBuilder { diff }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for VisitorBuilder<'s> {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        Box::new(Visitor::new(self.diff))
    }
}

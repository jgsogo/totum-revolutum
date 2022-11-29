use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};

use crate::diff::basepoint::{BasePoint, BasePointDiffImpl};
use crate::local::BasePointLocal;

use super::file_metadata::LocalFileMetadata;

pub struct Visitor<'s> {
    diff: &'s BasePointLocal,
}

impl<'s> Visitor<'s> {
    pub fn new(diff: &'s BasePointLocal) -> Visitor<'s> {
        Visitor { diff }
    }
}

impl<'s> ParallelVisitor for Visitor<'s> {
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> WalkState {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            self.diff.file_found(entry);
        }
        WalkState::Continue
    }
}

pub struct VisitorBuilder<'s> {
    diff: &'s BasePointLocal,
}

impl<'s> VisitorBuilder<'s> {
    pub fn new(diff: &BasePointLocal) -> VisitorBuilder {
        VisitorBuilder { diff }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for VisitorBuilder<'s> {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        Box::new(Visitor::new(self.diff))
    }
}

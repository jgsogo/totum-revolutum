use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};

use crate::diff::basepoint::BasePointDiffImpl;

use super::file_metadata::LocalFileMetadata;

pub struct Visitor<T: LocalFileMetadata> {
    base_path: std::path::PathBuf,
    diff: BasePointDiffImpl<T>,
}

impl<T> Visitor<T>
where
    T: LocalFileMetadata,
{
    pub fn new(base_path: &std::path::Path, diff: BasePointDiffImpl<T>) -> Visitor<T> {
        Visitor::<T> {
            diff,
            base_path: base_path.to_path_buf(),
        }
    }
}

impl<T> ParallelVisitor for Visitor<T>
where
    T: LocalFileMetadata,
{
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> ignore::WalkState {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            let metadata = <T as LocalFileMetadata>::from_direntry(&self.base_path, entry);
            self.diff.file_found(metadata);
        }
        WalkState::Continue
    }
}

pub struct VisitorBuilder<T: LocalFileMetadata> {
    base_path: std::path::PathBuf,
    diff: BasePointDiffImpl<T>,
}

impl<T> VisitorBuilder<T>
where
    T: LocalFileMetadata,
{
    pub fn new(base_path: &std::path::Path, diff: BasePointDiffImpl<T>) -> VisitorBuilder<T> {
        VisitorBuilder {
            diff,
            base_path: base_path.to_path_buf(),
        }
    }
}

impl<'s, T> ParallelVisitorBuilder<'s> for VisitorBuilder<T>
where
    T: LocalFileMetadata + 's,
{
    fn build(&mut self) -> Box<dyn ignore::ParallelVisitor + 's> {
        Box::new(Visitor::new(&self.base_path, self.diff.clone()))
    }
}

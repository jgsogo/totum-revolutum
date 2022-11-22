use std::path::PathBuf;

use crate::changes::{BasePointDiffImpl, FileMetadata};
use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};
use tracing::info;

pub struct Visitor<'a, T: FileMetadata> {
    diff: &'a BasePointDiffImpl<T>,
}

impl<'a, T> Visitor<'a, T>
where
    T: FileMetadata,
{
    pub fn new(diff: &'a BasePointDiffImpl<T>) -> Visitor<'a, T> {
        Visitor::<'a, T> { diff }
    }
}

impl<'a, T> ParallelVisitor for Visitor<'a, T>
where
    T: FileMetadata + std::marker::Send + From<ignore::DirEntry>,
{
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> ignore::WalkState {
        // println!("{}", entry.unwrap().path().display());
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            let metadata: T = entry.into();
            self.diff.file_found(metadata);
        }
        WalkState::Continue
    }
}

// impl Drop for Visitor<'_> {
//     fn drop(&mut self) {
//         info!("Files for this visitor");
//         for it in self.files.iter() {
//             println!("{}", it.display());
//         }
//     }
// }

pub struct VisitorBuilder<'a, T: FileMetadata> {
    diff: &'a BasePointDiffImpl<T>,
}

impl<'a, T> VisitorBuilder<'a, T>
where
    T: FileMetadata,
{
    pub fn new(diff: &'a BasePointDiffImpl<T>) -> VisitorBuilder<'a, T> {
        VisitorBuilder { diff }
    }
}

impl<'s, T> ParallelVisitorBuilder<'s> for VisitorBuilder<'s, T>
where
    T: FileMetadata + std::marker::Send + From<ignore::DirEntry>,
{
    fn build(&mut self) -> Box<dyn ignore::ParallelVisitor + 's> {
        Box::new(Visitor::new(self.diff))
    }
}

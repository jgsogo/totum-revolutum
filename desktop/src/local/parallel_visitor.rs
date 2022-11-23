use super::local::LocalFileMetadata;
use crate::actions::basepoint::BasePointDiffImpl;
use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};

pub struct Visitor<'a, T: LocalFileMetadata> {
    base_path: std::path::PathBuf,
    diff: &'a BasePointDiffImpl<T>,
}

impl<'a, T> Visitor<'a, T>
where
    T: LocalFileMetadata,
{
    pub fn new(base_path: &std::path::Path, diff: &'a BasePointDiffImpl<T>) -> Visitor<'a, T> {
        Visitor::<'a, T> {
            diff,
            base_path: base_path.to_path_buf(),
        }
    }
}

impl<'a, T> ParallelVisitor for Visitor<'a, T>
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

// impl Drop for Visitor<'_> {
//     fn drop(&mut self) {
//         info!("Files for this visitor");
//         for it in self.files.iter() {
//             println!("{}", it.display());
//         }
//     }
// }

pub struct VisitorBuilder<'a, T: LocalFileMetadata> {
    base_path: std::path::PathBuf,
    diff: &'a BasePointDiffImpl<T>,
}

impl<'a, T> VisitorBuilder<'a, T>
where
    T: LocalFileMetadata,
{
    pub fn new(
        base_path: &std::path::Path,
        diff: &'a BasePointDiffImpl<T>,
    ) -> VisitorBuilder<'a, T> {
        VisitorBuilder {
            diff,
            base_path: base_path.to_path_buf(),
        }
    }
}

impl<'s, T> ParallelVisitorBuilder<'s> for VisitorBuilder<'s, T>
where
    T: LocalFileMetadata,
{
    fn build(&mut self) -> Box<dyn ignore::ParallelVisitor + 's> {
        Box::new(Visitor::new(&self.base_path, self.diff))
    }
}

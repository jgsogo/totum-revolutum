use std::path::PathBuf;

use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};
use tracing::info;

#[derive(Default)]
pub struct Visitor {
    files: Vec<PathBuf>,
}

impl Visitor {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, 's> ParallelVisitorBuilder<'s> for Visitor {
    fn build(&mut self) -> Box<dyn ignore::ParallelVisitor + 's> {
        Box::new(Self {
            files: Vec::default(),
        })
    }
}

impl ParallelVisitor for Visitor {
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> ignore::WalkState {
        // println!("{}", entry.unwrap().path().display());
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            self.files.push(entry.path().to_path_buf());
        }
        WalkState::Continue
    }
}

impl Drop for Visitor {
    fn drop(&mut self) {
        info!("Files for this visitor");
        for it in self.files.iter() {
            println!("{}", it.display());
        }
    }
}

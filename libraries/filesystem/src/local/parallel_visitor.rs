use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};
use tracing::error;

use crate::local::LocalMetadata;

// FIXME: This doesn't probably belongs to this crate

pub struct Visitor {
    tx: flume::Sender<LocalMetadata>,
}

impl Visitor {
    pub fn new(tx: flume::Sender<LocalMetadata>) -> Visitor {
        Visitor { tx }
    }
}

impl ParallelVisitor for Visitor {
    fn visit(&mut self, entry: Result<ignore::DirEntry, ignore::Error>) -> WalkState {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            let data: LocalMetadata = entry.into();
            if let Err(e) = self.tx.send(data) {
                error!("Error sending direntry metadata: {e}. Quit visiting.");
                return WalkState::Quit;
            }
        }
        WalkState::Continue
    }
}

pub struct VisitorBuilder {
    tx: flume::Sender<LocalMetadata>,
}

impl VisitorBuilder {
    pub fn new(tx: flume::Sender<LocalMetadata>) -> VisitorBuilder {
        VisitorBuilder { tx }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for VisitorBuilder {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        Box::new(Visitor::new(self.tx.clone()))
    }
}

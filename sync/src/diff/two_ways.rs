use std::collections::HashMap;
use std::time::Instant;

use tracing::{error, info, trace};

use super::basepoint::{BasePointDiffImpl, FileMetadata, MAX_BUFFER};

pub struct FileDiff<LHS: FileMetadata, RHS: FileMetadata> {
    pub lhs: Option<LHS>,
    pub rhs: Option<RHS>,
}

impl<LHS, RHS> FileDiff<LHS, RHS>
where
    LHS: FileMetadata,
    RHS: FileMetadata,
{
    fn new(lhs: Option<LHS>, rhs: Option<RHS>) -> FileDiff<LHS, RHS> {
        FileDiff::<LHS, RHS> { lhs, rhs }
    }

    pub fn new_from_lhs(lhs: LHS) -> FileDiff<LHS, RHS> {
        Self::new(Some(lhs), None)
    }

    pub fn new_from_rhs(rhs: RHS) -> FileDiff<LHS, RHS> {
        Self::new(None, Some(rhs))
    }

    pub fn id(&self) -> &str {
        if let Some(v) = &self.lhs {
            return v.id();
        } else {
            self.rhs.as_ref().unwrap().id()
        }
    }
}

pub async fn run<LHS: FileMetadata + 'static, RHS: FileMetadata + 'static>() -> (
    flume::Sender<LHS>,
    flume::Sender<RHS>,
    flume::Receiver<FileDiff<LHS, RHS>>,
) {
    let (lhs_tx, lhs_rx) = flume::bounded::<LHS>(MAX_BUFFER);
    let (rhs_tx, rhs_rx) = flume::bounded::<RHS>(MAX_BUFFER);
    // let lhs = BasePointDiffImpl::<LHS>::new(lhs_tx);
    // let rhs = BasePointDiffImpl::<RHS>::new(rhs_tx);

    let (report_tx, report_rx) = flume::bounded::<FileDiff<LHS, RHS>>(MAX_BUFFER);

    tokio::spawn(async move {
        info!("Start receiving loop");
        let start = Instant::now();
        let mut files: HashMap<String, FileDiff<LHS, RHS>> = HashMap::new();
        loop {
            tokio::select! {
                Ok(lhs_metadata) = lhs_rx.recv_async() => {
                    trace!("LHS received {:?}", lhs_metadata);
                    let key = lhs_metadata.id().to_string();
                    match files.remove(&key) {
                        Some(mut v) => {
                            v.lhs = Some(lhs_metadata);
                            if let Err(e) = report_tx.send(v) {
                                error!("Receiving loop early stop. Report receiving end is lost: {e}");
                                break;
                            }
                        },
                        None => {
                            assert!(files.insert(key, FileDiff::<LHS, RHS>::new_from_lhs(lhs_metadata)).is_none());
                        }
                    }
                },
                Ok(rhs_metadata) = rhs_rx.recv_async() => {
                    trace!("RHS received {:?}", rhs_metadata);
                    let key = rhs_metadata.id().to_string();
                    match files.remove(&key) {
                        Some(mut v) => {
                            v.rhs = Some(rhs_metadata);
                            if let Err(e) = report_tx.send(v) {
                                error!("Receiving loop early stop. Report receiving end is lost: {e}");
                                break;
                            }
                        },
                        None => {
                            assert!(files.insert(key, FileDiff::<LHS, RHS>::new_from_rhs(rhs_metadata)).is_none());
                        }
                    }
                },
                else => {
                    info!("Break receiving loop in {:?}", start.elapsed());
                    break
                },
            }
        }
        // Now we need to send the files that are just on one side of the diff
        trace!("Send remaining {} entries", files.len());
        for (_, file_diff) in files.drain() {
            if let Err(e) = report_tx.send(file_diff) {
                error!("Send error {e}");
            }
        }
        info!("Finished receiving loop in {:?}", start.elapsed());

        // TODO: We should never have duplicated files on either local or remote (the same id several
        //  times in the same local or remote), but shit happens. If someone is operating with the files
        //  while we are working on them, it can happen that we inform about some file, then remove it
        //  and that process adds it again and our visitors see it one more time. Is this possible?
    });

    (lhs_tx, rhs_tx, report_rx)
}

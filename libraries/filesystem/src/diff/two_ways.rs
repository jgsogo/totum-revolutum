use std::collections::HashMap;
use std::time::Instant;

use tracing::{debug, error, info, trace};

use crate::FileMetadata;

use super::file_pair::FilePair;

const MAX_BUFFER: usize = 100;

/// Creates the channels for a two ways diffs and implements the main algorithm. Returns
/// the endpoints for:
///  * `lhs_sender`: this endpoint should be used by the LHS [`Filesystem`] to send
///     the [`FileMetadata`] for the files in the working directory
///  * `rhs_sender`: this endpoint should be used by the RHS [`Filesystem`] to send
///     the [`FileMetadata`] for the files in the working directory
///  * `file_pair_receiver`: this endpoint will consume the [`FilePair`]s tuples composed based on the
///     inputs of the other two senders. It will also receive orphan pairs, that is, files that appears
///     just on one of the filesystems.
pub async fn run<LHS: FileMetadata + 'static, RHS: FileMetadata + 'static>() -> (
    flume::Sender<LHS>,
    flume::Sender<RHS>,
    flume::Receiver<FilePair<LHS, RHS>>,
) {
    let (lhs_tx, lhs_rx) = flume::bounded::<LHS>(MAX_BUFFER);
    let (rhs_tx, rhs_rx) = flume::bounded::<RHS>(MAX_BUFFER);

    let (report_tx, report_rx) = flume::bounded::<FilePair<LHS, RHS>>(MAX_BUFFER);

    tokio::spawn(async move {
        info!("Start receiving loop");
        let start = Instant::now();
        let mut files: HashMap<String, FilePair<LHS, RHS>> = HashMap::new();
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
                            let _ = files.insert(key, FilePair::<LHS, RHS>::new_from_lhs(lhs_metadata));
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
                            let _ = files.insert(key, FilePair::<LHS, RHS>::new_from_rhs(rhs_metadata));
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
        debug!("Send remaining {} entries", files.len());
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

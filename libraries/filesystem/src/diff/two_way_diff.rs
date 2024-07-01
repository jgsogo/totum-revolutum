use std::collections::HashMap;
use std::time::Instant;

use ignore_files::IgnoreFilter;
use tracing::{debug, error, info, trace};

use crate::diff::receiver::{FileMetadataPair, Receiver};
use crate::{FileMetadata, Filesystem, Result};

const MAX_BUFFER: usize = 100;

/// Executes 2-way diff algorithm. It goes through **all the files in both filesystems** and sends
/// the information to the provided [`Receiver`].
///
/// It will typically require the receiver to record the operations that need to be done in the
/// filesystems and apply them afterward.
///
/// Complexity of this algorithm is `O(N+M + log P)` where `N` and `M` are the number of files in
/// each filesystem and `log P` represents an overhead introduced by the index that is used to
/// cross the information from both filesystems. Worst case scenario for this index is to contain
/// all the entries from the larger filesystem before any data arrives from the other.
pub async fn full_run<LHSFilesystem: Filesystem, RHSFilesystem: Filesystem, TReceiver: Receiver>(
    lhs_filesystem: &LHSFilesystem,
    rhs_filesystem: &RHSFilesystem,
    receiver: &mut TReceiver,
    lhs_ignore_filter: IgnoreFilter,
    rhs_ignore_filter: IgnoreFilter,
) -> Result<()> {
    let (lhs_tx, lhs_rx) = flume::bounded::<FileMetadata>(MAX_BUFFER);
    let (rhs_tx, rhs_rx) = flume::bounded::<FileMetadata>(MAX_BUFFER);

    let (report_tx, report_rx) = flume::bounded::<FileMetadataPair>(MAX_BUFFER);

    // NOTE: We should never have duplicated files on either local or remote (the same id several
    //  times in the same local or remote), but shit happens. If someone is operating with the files
    //  while we are working on them, it can happen that we inform about some file, then remove it
    //  and that process adds it again and our visitors see it one more time. Is this possible?

    tokio::spawn(async move {
        info!("Start 2-way diff sending loop");
        let start = Instant::now();
        let mut files: HashMap<String, FileMetadataPair> = HashMap::new();
        loop {
            tokio::select! {
                Ok(lhs_metadata) = lhs_rx.recv_async() => {
                    trace!("LHS received {:?}", lhs_metadata);
                    let key = lhs_metadata.path().as_str();
                    match files.remove(key) {
                        Some((_, rhs_metadata)) => {
                            if let Err(e) = report_tx.send((Some(lhs_metadata), rhs_metadata)) {
                                error!("Sending loop early stop. Report receiving end is lost: {e}");
                                break;
                            }
                        },
                        None => {
                            let _ = files.insert(key.to_string(), (Some(lhs_metadata), None));
                        }
                    }
                },
                Ok(rhs_metadata) = rhs_rx.recv_async() => {
                    trace!("RHS received {:?}", rhs_metadata);
                    let key = rhs_metadata.path().as_str();
                    match files.remove(key) {
                        Some((lhs_metadata, _)) => {
                            if let Err(e) = report_tx.send((lhs_metadata, Some(rhs_metadata))) {
                                error!("Sending loop early stop. Report receiving end is lost: {e}");
                                break;
                            }
                        },
                        None => {
                            let _ = files.insert(key.to_string(), (None, Some(rhs_metadata)));
                        }
                    }
                },
                else => {
                    info!("Break sending loop in {:?}", start.elapsed());
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
        info!("Finished sending loop in {:?}", start.elapsed());
    });

    let _ = tokio::try_join!(
        lhs_filesystem.walk_directory(lhs_tx, lhs_ignore_filter),
        rhs_filesystem.walk_directory(rhs_tx, rhs_ignore_filter),
        work_on_results(report_rx, receiver),
    )?;

    Ok(())
}

async fn work_on_results<TReceiver: Receiver>(
    rx: flume::Receiver<FileMetadataPair>,
    receiver: &mut TReceiver,
) -> Result<()> {
    debug!("Start 2-way diff receiving loop");
    let start = Instant::now();
    while let Ok(file_pair) = rx.recv_async().await {
        receiver.on_data(file_pair).await?;
    }
    debug!("Finished 2-way diff receiving loop in {:?}", start.elapsed());
    Ok(())
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::diff::tests::DiffMocks;

    use super::*;

    #[derive(Default)]
    pub(crate) struct ReceiverMock {
        pub only_lhs: Vec<String>,
        pub only_rhs: Vec<String>,
        pub equal: Vec<String>,
        pub diff: Vec<String>,
    }

    #[async_trait]
    impl Receiver for ReceiverMock {
        async fn only_lhs(&mut self, file_metadata: FileMetadata) -> Result<()> {
            self.only_lhs.push(file_metadata.path().to_string());
            Ok(())
        }
        async fn only_rhs(&mut self, file_metadata: FileMetadata) -> Result<()> {
            self.only_rhs.push(file_metadata.path().to_string());
            Ok(())
        }
        async fn equal_files(
            &mut self,
            lhs_file_metadata: FileMetadata,
            rhs_file_metadata: FileMetadata,
        ) -> Result<()> {
            assert_eq!(lhs_file_metadata.path(), rhs_file_metadata.path());
            assert_eq!(lhs_file_metadata.size(), rhs_file_metadata.size());
            assert_eq!(lhs_file_metadata.hash(), rhs_file_metadata.hash());
            self.equal.push(lhs_file_metadata.path().to_string());
            Ok(())
        }
        async fn diff_files(&mut self, lhs_file_metadata: FileMetadata, rhs_file_metadata: FileMetadata) -> Result<()> {
            assert_eq!(lhs_file_metadata.path(), rhs_file_metadata.path());
            self.diff.push(lhs_file_metadata.path().to_string());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_full_run() -> Result<()> {
        let diff_mocks = DiffMocks::new().await?;

        let mut receiver = ReceiverMock::default();
        full_run(
            &diff_mocks.fs_lhs,
            &diff_mocks.fs_rhs,
            &mut receiver,
            IgnoreFilter::empty(""),
            IgnoreFilter::empty(""),
        )
        .await?;

        assert_eq!(receiver.only_lhs.len(), 1);
        assert_eq!(receiver.only_lhs.get(0).unwrap(), diff_mocks.lhs_only.as_str());

        assert_eq!(receiver.only_rhs.len(), 1);
        assert_eq!(receiver.only_rhs.get(0).unwrap(), diff_mocks.rhs_only.as_str());

        assert_eq!(receiver.equal.len(), 1);
        assert_eq!(receiver.equal.get(0).unwrap(), diff_mocks.both_equal.as_str());

        assert_eq!(receiver.diff.len(), 2);
        receiver.diff.sort();
        assert_eq!(receiver.diff.get(0).unwrap(), diff_mocks.diff_hash.as_str());
        assert_eq!(receiver.diff.get(1).unwrap(), diff_mocks.diff_size.as_str());

        Ok(())
    }
}

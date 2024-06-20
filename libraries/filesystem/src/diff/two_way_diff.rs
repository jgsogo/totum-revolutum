use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Instant;

use camino::Utf8Path;
use tracing::{debug, error, info, trace};

use crate::{FileMetadata, Filesystem, Result};

const MAX_BUFFER: usize = 100;

pub type FileMetadataPair = (Option<Box<dyn FileMetadata>>, Option<Box<dyn FileMetadata>>);

/// Interface to receive the results from the 2-way diff [`run`] function
#[async_trait]
pub trait Receiver: Sync {
    /// Receives every [`FileMetadataPair`] from the filesystems we are iterating.
    ///
    /// The default implementation will just forward the call to the right method from
    /// [`Receiver::only_lhs`], [`Receiver::only_rhs`] or [`Receiver::lhs_and_rhs`].
    async fn on_data(&self, data: FileMetadataPair) -> Result<()> {
        match data {
            (Some(lhs), None) => self.only_lhs(lhs).await,
            (None, Some(rhs)) => self.only_rhs(rhs).await,
            (Some(lhs), Some(rhs)) => self.lhs_and_rhs(lhs, rhs).await,
            (None, None) => unreachable!(),
        }
    }

    /// Receives the [`FileMetadata`] for the files that are only present in the left-hand-side
    /// filesystem
    async fn only_lhs(&self, _file_metadata: Box<dyn FileMetadata>) -> Result<()> {
        Ok(())
    }

    /// Receives the [`FileMetadata`] for the files that are only present in the right-hand-side
    /// filesystem
    async fn only_rhs(&self, _file_metadata: Box<dyn FileMetadata>) -> Result<()> {
        Ok(())
    }

    /// Receives the [`FileMetadata`] for the files that are present in both filesystems.
    ///
    /// The default implementation will forward the call to [`Receiver::equal_files`] or
    /// [`Receiver::diff_files`].
    async fn lhs_and_rhs(
        &self,
        lhs_file_metadata: Box<dyn FileMetadata>,
        rhs_file_metadata: Box<dyn FileMetadata>,
    ) -> Result<()> {
        if lhs_file_metadata.eq(rhs_file_metadata.as_ref())? {
            self.equal_files(lhs_file_metadata, rhs_file_metadata).await
        } else {
            self.diff_files(lhs_file_metadata, rhs_file_metadata).await
        }
    }

    /// Receives the [`FileMetadata`] for the files that are present in both filesystems and are equal
    async fn equal_files(
        &self,
        _lhs_file_metadata: Box<dyn FileMetadata>,
        _rhs_file_metadata: Box<dyn FileMetadata>,
    ) -> Result<()> {
        Ok(())
    }

    /// Receives the [`FileMetadata`] for the files that are present in both filesystems and are different
    async fn diff_files(
        &self,
        _lhs_file_metadata: Box<dyn FileMetadata>,
        _rhs_file_metadata: Box<dyn FileMetadata>,
    ) -> Result<()> {
        Ok(())
    }
}

async fn work_on_results<TReceiver: Receiver>(
    rx: flume::Receiver<FileMetadataPair>,
    receiver: &TReceiver,
) -> Result<()> {
    debug!("Start 2-way diff receiving loop");
    let start = Instant::now();
    while let Ok(file_pair) = rx.recv_async().await {
        receiver.on_data(file_pair).await?;
    }
    debug!("Finished 2-way diff receiving loop in {:?}", start.elapsed());
    Ok(())
}

/// Executes 2-way diff algorithm. It goes through all the files in both filesystems and send
/// the information to the provided [`Receiver`]
pub async fn run<LHSFilesystem: Filesystem, RHSFilesystem: Filesystem, TReceiver: Receiver>(
    lhs_filesystem: &LHSFilesystem,
    rhs_filesystem: &RHSFilesystem,
    receiver: &TReceiver,
) -> Result<()> {
    let (lhs_tx, lhs_rx) = flume::bounded::<Box<dyn FileMetadata>>(MAX_BUFFER);
    let (rhs_tx, rhs_rx) = flume::bounded::<Box<dyn FileMetadata>>(MAX_BUFFER);

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
        lhs_filesystem.walk_directory(lhs_tx, 6, Utf8Path::new("/")),
        rhs_filesystem.walk_directory(rhs_tx, 6, Utf8Path::new("/")),
        work_on_results(report_rx, receiver),
    )?;

    Ok(())
}

use std::collections::HashMap;
use std::time::Instant;

use async_trait::async_trait;
use camino::Utf8Path;
use tracing::{debug, error, info, trace};

use crate::{FileMetadata, Filesystem, Result};

const MAX_BUFFER: usize = 100;

pub type FileMetadataPair = (Option<Box<dyn FileMetadata>>, Option<Box<dyn FileMetadata>>);

/// Interface to receive the results from the 2-way diff [`run`] function
#[async_trait]
pub trait Receiver: Send + Sync {
    /// Receives every [`FileMetadataPair`] from the filesystems we are iterating.
    ///
    /// The default implementation will just forward the call to the right method from
    /// [`Receiver::only_lhs`], [`Receiver::only_rhs`] or [`Receiver::lhs_and_rhs`].
    async fn on_data(&mut self, data: FileMetadataPair) -> Result<()> {
        match data {
            (Some(lhs), None) => self.only_lhs(lhs).await,
            (None, Some(rhs)) => self.only_rhs(rhs).await,
            (Some(lhs), Some(rhs)) => self.lhs_and_rhs(lhs, rhs).await,
            (None, None) => unreachable!(),
        }
    }

    /// Receives the [`FileMetadata`] for the files that are only present in the left-hand-side
    /// filesystem
    async fn only_lhs(&mut self, _file_metadata: Box<dyn FileMetadata>) -> Result<()> {
        Ok(())
    }

    /// Receives the [`FileMetadata`] for the files that are only present in the right-hand-side
    /// filesystem
    async fn only_rhs(&mut self, _file_metadata: Box<dyn FileMetadata>) -> Result<()> {
        Ok(())
    }

    /// Receives the [`FileMetadata`] for the files that are present in both filesystems.
    ///
    /// The default implementation will forward the call to [`Receiver::equal_files`] or
    /// [`Receiver::diff_files`].
    async fn lhs_and_rhs(
        &mut self,
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
        &mut self,
        _lhs_file_metadata: Box<dyn FileMetadata>,
        _rhs_file_metadata: Box<dyn FileMetadata>,
    ) -> Result<()> {
        Ok(())
    }

    /// Receives the [`FileMetadata`] for the files that are present in both filesystems and are different
    async fn diff_files(
        &mut self,
        _lhs_file_metadata: Box<dyn FileMetadata>,
        _rhs_file_metadata: Box<dyn FileMetadata>,
    ) -> Result<()> {
        Ok(())
    }
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

/// Executes 2-way diff algorithm. It goes through all the files in both filesystems and send
/// the information to the provided [`Receiver`]
pub async fn run<LHSFilesystem: Filesystem, RHSFilesystem: Filesystem, TReceiver: Receiver>(
    lhs_filesystem: &LHSFilesystem,
    rhs_filesystem: &RHSFilesystem,
    receiver: &mut TReceiver,
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::impls::FilesystemLocalTemp;
    use crate::wrappers::AsyncFileDropImpl;
    use crate::{DirectoryPathBuf, Error, FilePath, FilenameBuf};

    use super::*;

    #[derive(Default)]
    struct ReceiverMock {
        pub only_lhs: Vec<String>,
        pub only_rhs: Vec<String>,
        pub equal: Vec<String>,
        pub diff: Vec<String>,
    }

    #[async_trait]
    impl Receiver for ReceiverMock {
        async fn only_lhs(&mut self, file_metadata: Box<dyn FileMetadata>) -> Result<()> {
            self.only_lhs.push(file_metadata.path().to_string());
            Ok(())
        }
        async fn only_rhs(&mut self, file_metadata: Box<dyn FileMetadata>) -> Result<()> {
            self.only_rhs.push(file_metadata.path().to_string());
            Ok(())
        }
        async fn equal_files(
            &mut self,
            lhs_file_metadata: Box<dyn FileMetadata>,
            rhs_file_metadata: Box<dyn FileMetadata>,
        ) -> Result<()> {
            assert_eq!(lhs_file_metadata.path(), rhs_file_metadata.path());
            assert_eq!(lhs_file_metadata.size()?, rhs_file_metadata.size()?);
            assert_eq!(lhs_file_metadata.hash()?, rhs_file_metadata.hash()?);
            self.equal.push(lhs_file_metadata.path().to_string());
            Ok(())
        }
        async fn diff_files(
            &mut self,
            lhs_file_metadata: Box<dyn FileMetadata>,
            rhs_file_metadata: Box<dyn FileMetadata>,
        ) -> Result<()> {
            assert_eq!(lhs_file_metadata.path(), rhs_file_metadata.path());
            self.diff.push(lhs_file_metadata.path().to_string());
            Ok(())
        }
    }

    async fn create_file(filesystem: &mut dyn Filesystem, path: &FilePath, content: &[u8]) -> Result<()> {
        let rx = {
            let (mut f, rx) = filesystem.create(path).await?;
            f.write_all(content).await?;
            rx.unwrap()
        };

        rx.await.map_err(|e| Error::Other(e.to_string()))??;
        Ok(())
    }

    #[tokio::test]
    async fn test_run() -> Result<()> {
        let root = DirectoryPathBuf::root();
        let lhs_only = root.join_filename(FilenameBuf::from_str("lhs_only")?);
        let rhs_only = root.join_filename(FilenameBuf::from_str("rhs_only")?);
        let both_equal = root.join_filename(FilenameBuf::from_str("both_equal")?);
        let diff_hash = root.join_filename(FilenameBuf::from_str("diff_hash")?);
        let diff_size = root.join_filename(FilenameBuf::from_str("diff_size")?);

        let fs_lhs = {
            let mut fs = AsyncFileDropImpl::new_call_sync_all(FilesystemLocalTemp::default());
            create_file(&mut fs, &lhs_only, b"anything").await?;
            create_file(&mut fs, &both_equal, b"both_equal").await?;
            create_file(&mut fs, &diff_hash, b"lhs_hash").await?;
            create_file(&mut fs, &diff_size, b"lhs size").await?;
            fs
        };

        let fs_rhs = {
            let mut fs = AsyncFileDropImpl::new_call_sync_all(FilesystemLocalTemp::default());
            create_file(&mut fs, &rhs_only, b"anything").await?;
            create_file(&mut fs, &both_equal, b"both_equal").await?;
            create_file(&mut fs, &diff_hash, b"rhs_hash").await?;
            create_file(&mut fs, &diff_size, b"rhs size so it's different").await?;
            fs
        };

        let mut receiver = ReceiverMock::default();
        run(&fs_lhs, &fs_rhs, &mut receiver).await?;

        assert_eq!(receiver.only_lhs.len(), 1);
        assert_eq!(receiver.only_lhs.get(0).unwrap(), lhs_only.as_str());

        assert_eq!(receiver.only_rhs.len(), 1);
        assert_eq!(receiver.only_rhs.get(0).unwrap(), rhs_only.as_str());

        assert_eq!(receiver.equal.len(), 1);
        assert_eq!(receiver.equal.get(0).unwrap(), both_equal.as_str());

        assert_eq!(receiver.diff.len(), 2);
        receiver.diff.sort();
        assert_eq!(receiver.diff.get(0).unwrap(), diff_hash.as_str());
        assert_eq!(receiver.diff.get(1).unwrap(), diff_size.as_str());

        Ok(())
    }
}

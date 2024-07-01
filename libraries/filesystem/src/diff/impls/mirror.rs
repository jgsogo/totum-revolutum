use async_trait::async_trait;
use ignore_files::IgnoreFilter;
use tracing::{debug, error};

use crate::diff::two_way_diff::full_run;
use crate::diff::Receiver;
use crate::{Error, FileMetadata, Filesystem, FilesystemOps, Result};

#[derive(Default)]
struct MirrorReceiver {
    to_copy: Vec<Box<dyn FileMetadata>>,
    to_override: Vec<(Box<dyn FileMetadata>, Box<dyn FileMetadata>)>,
    to_delete: Vec<Box<dyn FileMetadata>>,
}

#[async_trait]
impl Receiver for MirrorReceiver {
    async fn only_lhs(&mut self, file_metadata: Box<dyn FileMetadata>) -> Result<()> {
        self.to_copy.push(file_metadata);
        Ok(())
    }

    async fn only_rhs(&mut self, file_metadata: Box<dyn FileMetadata>) -> Result<()> {
        self.to_delete.push(file_metadata);
        Ok(())
    }

    async fn diff_files(
        &mut self,
        lhs_file_metadata: Box<dyn FileMetadata>,
        rhs_file_metadata: Box<dyn FileMetadata>,
    ) -> Result<()> {
        self.to_override.push((lhs_file_metadata, rhs_file_metadata));
        Ok(())
    }
}

/// Modifies `rhs_filesystem` so it contains exactly the same files as `lhs_filesystem`:
///  * Files that are not present in `rhs_filesystem` will be copied.
///  * Conflicting files will be overridden.
///  * Files in `rhs_filesystem` that are not in `lhs_filesystem` will be removed.
///
/// TODO: What about directories? How/where/when we remove empty directories? Is it a different `clean` operation?
pub async fn mirror<LHSFilesystem: Filesystem, RHSFilesystem: FilesystemOps>(
    lhs_filesystem: &LHSFilesystem,
    rhs_filesystem: &mut RHSFilesystem,
    lhs_ignore_file: IgnoreFilter,
) -> Result<()> {
    let mut mirror_receiver = MirrorReceiver::default();
    full_run(
        lhs_filesystem,
        rhs_filesystem,
        &mut mirror_receiver,
        lhs_ignore_file,
        IgnoreFilter::empty(""),
    )
    .await?;

    let mut set = tokio::task::JoinSet::new();

    debug!("Copy missing files to the mirror");
    for file in mirror_receiver.to_copy {
        let rx = rhs_filesystem
            .copy_from(file.path(), lhs_filesystem, file.path(), true)
            .await?;
        if let Some(rx) = rx {
            set.spawn(rx);
        }
    }

    debug!("Override files when there is a conflict");
    for (lhs_metadata, rhs_metadata) in mirror_receiver.to_override {
        assert_eq!(lhs_metadata.path(), rhs_metadata.path());
        let rx = rhs_filesystem
            .copy_from(lhs_metadata.path(), lhs_filesystem, lhs_metadata.path(), true)
            .await?;
        if let Some(rx) = rx {
            set.spawn(rx);
        }
    }

    debug!("Remove the files that are not in the source");
    for file in mirror_receiver.to_delete {
        rhs_filesystem.remove_file(file.path()).await?;
    }

    debug!("Wait for all the work to finish");
    while let Some(r) = set.join_next().await {
        if let Err(e) = r
            .map_err(|e| Error::Other(e.to_string()))?
            .map_err(|e| Error::Other(e.to_string()))?
        {
            error!("Error executing backup work: {e}");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::tests::DiffMocks;

    #[tokio::test]
    async fn test_mirror() -> Result<()> {
        let mut diff_mocks = DiffMocks::new().await?;

        // pre-conditions
        assert!(!diff_mocks.fs_rhs.exists(&diff_mocks.lhs_only).await?);
        assert!(diff_mocks.fs_rhs.exists(&diff_mocks.rhs_only).await?);
        let original_diff_hash_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_hash).await?;
        let original_diff_size_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_size).await?;

        mirror(&diff_mocks.fs_lhs, &mut diff_mocks.fs_rhs, IgnoreFilter::empty("")).await?;

        // checks
        assert!(diff_mocks.fs_rhs.exists(&diff_mocks.lhs_only).await?);
        assert!(!diff_mocks.fs_rhs.exists(&diff_mocks.rhs_only).await?);
        {
            // diff_hash file has changed
            let rhs_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_hash).await?;
            assert!(!rhs_metadata.eq(original_diff_hash_metadata.as_ref())?);
            let lhs_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.diff_hash).await?;
            assert!(rhs_metadata.eq(lhs_metadata.as_ref())?);
        }
        {
            // diff_size file now is equal
            let rhs_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_size).await?;
            assert!(!rhs_metadata.eq(original_diff_size_metadata.as_ref())?);
            let lhs_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.diff_size).await?;
            assert!(rhs_metadata.eq(lhs_metadata.as_ref())?);
        }

        Ok(())
    }
}

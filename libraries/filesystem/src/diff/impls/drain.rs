use async_trait::async_trait;
use ignore_files::IgnoreFilter;

use crate::diff::two_way_diff::full_run;
use crate::diff::Receiver;
use crate::{FileMetadata, Filesystem, FilesystemOps, Result};

/// Action to take when there is a conflict executing a [`drain`]
pub enum DrainConflict {
    /// Do not remove. Keep the local version of the file
    Keep,

    /// Remove the local version of the file
    Remove,
}

#[derive(Default)]
struct DrainReceiver {
    to_remove: Vec<(FileMetadata, FileMetadata)>,
    conflicts: Vec<(FileMetadata, FileMetadata)>,
}

#[async_trait]
impl Receiver for DrainReceiver {
    async fn equal_files(&mut self, lhs_file_metadata: FileMetadata, rhs_file_metadata: FileMetadata) -> Result<()> {
        self.to_remove.push((lhs_file_metadata, rhs_file_metadata));
        Ok(())
    }

    async fn diff_files(&mut self, lhs_file_metadata: FileMetadata, rhs_file_metadata: FileMetadata) -> Result<()> {
        self.conflicts.push((lhs_file_metadata, rhs_file_metadata));
        Ok(())
    }
}

/// Removes from `lhs_filesystem` all the files that are already present in `rhs_filesystem`. Use
/// `on_conflict` argument to choose what to do if the file is already there but they are not
/// equal.
pub async fn drain<LHSFilesystem: Filesystem, RHSFilesystem: FilesystemOps>(
    lhs_filesystem: &mut LHSFilesystem,
    rhs_filesystem: &RHSFilesystem,
    on_conflict: DrainConflict,
    rhs_ignore_file: IgnoreFilter,
) -> Result<()> {
    let mut drain_receiver = DrainReceiver::default();
    full_run(
        lhs_filesystem,
        rhs_filesystem,
        &mut drain_receiver,
        IgnoreFilter::empty(""),
        rhs_ignore_file,
    )
    .await?;

    // Remove files that are already in the target
    for (lhs_metadata, rhs_metadata) in drain_receiver.to_remove {
        assert_eq!(lhs_metadata.path(), rhs_metadata.path());
        lhs_filesystem.remove_file(lhs_metadata.path()).await?;
    }

    // Remove or keep conflicts
    match on_conflict {
        DrainConflict::Remove => {
            for (lhs_metadata, rhs_metadata) in drain_receiver.conflicts {
                assert_eq!(lhs_metadata.path(), rhs_metadata.path());
                lhs_filesystem.remove_file(lhs_metadata.path()).await?;
            }
        }
        DrainConflict::Keep => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::diff::tests::DiffMocks;

    use super::*;

    #[tokio::test]
    async fn test_drain_keep() -> Result<()> {
        let mut diff_mocks = DiffMocks::new().await?;

        // pre-conditions
        assert!(diff_mocks.fs_lhs.exists(&diff_mocks.both_equal).await?);
        let original_diff_hash_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.diff_hash).await?;
        let original_diff_size_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.diff_size).await?;

        drain(
            &mut diff_mocks.fs_lhs,
            &diff_mocks.fs_rhs,
            DrainConflict::Keep,
            IgnoreFilter::empty(""),
        )
        .await?;

        // checks
        {
            // both_equal file has been removed
            assert!(!diff_mocks.fs_lhs.exists(&diff_mocks.both_equal).await?);
            // lhs_only is still there
            assert!(diff_mocks.fs_lhs.exists(&diff_mocks.lhs_only).await?);
        }
        {
            // diff_hash file has NOT changed
            let lhs_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.diff_hash).await?;
            assert_eq!(lhs_metadata, original_diff_hash_metadata);
        }
        {
            // diff_size file has NOT changed
            let lhs_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.diff_size).await?;
            assert_eq!(lhs_metadata, original_diff_size_metadata);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_drain_remove() -> Result<()> {
        let mut diff_mocks = DiffMocks::new().await?;

        // pre-conditions
        assert!(diff_mocks.fs_lhs.exists(&diff_mocks.both_equal).await?);
        assert!(diff_mocks.fs_lhs.exists(&diff_mocks.diff_hash).await?);
        assert!(diff_mocks.fs_lhs.exists(&diff_mocks.diff_size).await?);

        drain(
            &mut diff_mocks.fs_lhs,
            &diff_mocks.fs_rhs,
            DrainConflict::Remove,
            IgnoreFilter::empty(""),
        )
        .await?;

        // checks
        assert!(!diff_mocks.fs_lhs.exists(&diff_mocks.both_equal).await?);
        assert!(!diff_mocks.fs_lhs.exists(&diff_mocks.diff_hash).await?);
        assert!(!diff_mocks.fs_lhs.exists(&diff_mocks.diff_size).await?);
        assert!(diff_mocks.fs_lhs.exists(&diff_mocks.lhs_only).await?);

        Ok(())
    }
}

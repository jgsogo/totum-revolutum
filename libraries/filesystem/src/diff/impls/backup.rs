use async_trait::async_trait;
use ignore_files::IgnoreFilter;
use tracing::error;

use crate::diff::two_way_diff::full_run;
use crate::diff::Receiver;
use crate::{Error, FileMetadata, Filesystem, FilesystemOps, Result};

/// Action to take when there is a conflict executing a [`backup`]
pub enum BackupConflict {
    /// Override the target file with the contents of the origin
    Override,

    // TODO: Implement rename (decide rename strategy)
    // Rename,
    /// Ignore the conflict
    Skip,
}

#[derive(Default)]
struct BackupReceiver {
    to_copy: Vec<FileMetadata>,
    conflicts: Vec<(FileMetadata, FileMetadata)>,
}

#[async_trait]
impl Receiver for BackupReceiver {
    async fn only_lhs(&mut self, file_metadata: FileMetadata) -> Result<()> {
        self.to_copy.push(file_metadata);
        Ok(())
    }

    async fn diff_files(&mut self, lhs_file_metadata: FileMetadata, rhs_file_metadata: FileMetadata) -> Result<()> {
        self.conflicts.push((lhs_file_metadata, rhs_file_metadata));
        Ok(())
    }
}

/// Copies to `rhs_filesystem` all the files from `lhs_filesystem` that are not already there. Use
/// the argument `on_conflict` to choose what to do with conflicting files.
pub async fn backup<LHSFilesystem: Filesystem, RHSFilesystem: FilesystemOps>(
    lhs_filesystem: &LHSFilesystem,
    rhs_filesystem: &mut RHSFilesystem,
    on_conflict: BackupConflict,
    lhs_ignore_filter: IgnoreFilter,
) -> Result<()> {
    let mut backup_receiver = BackupReceiver::default();
    full_run(
        lhs_filesystem,
        rhs_filesystem,
        &mut backup_receiver,
        lhs_ignore_filter,
        IgnoreFilter::empty(""),
    )
    .await?;

    let mut set = tokio::task::JoinSet::new();

    // Copy missing files to the backup
    for file in backup_receiver.to_copy {
        let rx = rhs_filesystem
            .copy_from(file.path(), lhs_filesystem, file.path(), true)
            .await?;
        if let Some(rx) = rx {
            set.spawn(rx);
        }
    }

    // Override (or ignore) conflicts
    match on_conflict {
        BackupConflict::Override => {
            for (lhs_metadata, rhs_metadata) in backup_receiver.conflicts {
                assert_eq!(lhs_metadata.path(), rhs_metadata.path());
                let rx = rhs_filesystem
                    .copy_from(lhs_metadata.path(), lhs_filesystem, lhs_metadata.path(), true)
                    .await?;
                if let Some(rx) = rx {
                    set.spawn(rx);
                }
            }
        }
        BackupConflict::Skip => {}
    }

    // Wait for all the work to finish
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
    use crate::diff::tests::DiffMocks;

    use super::*;

    #[tokio::test]
    async fn test_backup_override() -> Result<()> {
        let mut diff_mocks = DiffMocks::new().await?;

        // pre-conditions
        assert!(!diff_mocks.fs_rhs.exists(&diff_mocks.lhs_only).await?);
        let original_diff_hash_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_hash).await?;
        let original_diff_size_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_size).await?;

        backup(
            &diff_mocks.fs_lhs,
            &mut diff_mocks.fs_rhs,
            BackupConflict::Override,
            IgnoreFilter::empty(""),
        )
        .await?;

        // checks
        {
            // lhs_only file has been copied to the target filesystem
            assert!(diff_mocks.fs_rhs.exists(&diff_mocks.lhs_only).await?);
            let rhs_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.lhs_only).await?;
            let lhs_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.lhs_only).await?;
            assert_eq!(rhs_metadata, lhs_metadata);
        }
        {
            // diff_hash file has changed
            let rhs_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_hash).await?;
            assert_ne!(rhs_metadata, original_diff_hash_metadata);
            let lhs_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.diff_hash).await?;
            assert_eq!(rhs_metadata, lhs_metadata);
        }
        {
            // diff_size file now is equal
            let rhs_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_size).await?;
            assert_ne!(rhs_metadata, original_diff_size_metadata);
            let lhs_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.diff_size).await?;
            assert_eq!(rhs_metadata, lhs_metadata);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_backup_skip() -> Result<()> {
        let mut diff_mocks = DiffMocks::new().await?;

        // pre-conditions
        assert!(!diff_mocks.fs_rhs.exists(&diff_mocks.lhs_only).await?);
        let original_diff_hash_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_hash).await?;
        let original_diff_size_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_size).await?;

        backup(
            &diff_mocks.fs_lhs,
            &mut diff_mocks.fs_rhs,
            BackupConflict::Skip,
            IgnoreFilter::empty(""),
        )
        .await?;

        // checks
        {
            // lhs_only file has been copied to the target filesystem
            assert!(diff_mocks.fs_rhs.exists(&diff_mocks.lhs_only).await?);
            let rhs_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.lhs_only).await?;
            let lhs_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.lhs_only).await?;
            assert_eq!(rhs_metadata, lhs_metadata);
        }
        {
            // diff_hash file has NOT changed
            let rhs_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_hash).await?;
            assert_eq!(rhs_metadata, original_diff_hash_metadata);
            let lhs_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.diff_hash).await?;
            assert_ne!(rhs_metadata, lhs_metadata);
        }
        {
            // diff_size file has NOT changed
            let rhs_metadata = diff_mocks.fs_rhs.get_metadata(&diff_mocks.diff_size).await?;
            assert_eq!(rhs_metadata, original_diff_size_metadata);
            let lhs_metadata = diff_mocks.fs_lhs.get_metadata(&diff_mocks.diff_size).await?;
            assert_ne!(rhs_metadata, lhs_metadata);
        }

        Ok(())
    }
}

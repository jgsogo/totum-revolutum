use crate::{FileMetadata, Result};
use async_trait::async_trait;

pub type FileMetadataPair = (Option<FileMetadata>, Option<FileMetadata>);

/// Interface to receive the results from the 2-way diff [`super::full_run`] function
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
    async fn only_lhs(&mut self, _file_metadata: FileMetadata) -> Result<()> {
        Ok(())
    }

    /// Receives the [`FileMetadata`] for the files that are only present in the right-hand-side
    /// filesystem
    async fn only_rhs(&mut self, _file_metadata: FileMetadata) -> Result<()> {
        Ok(())
    }

    /// Receives the [`FileMetadata`] for the files that are present in both filesystems.
    ///
    /// The default implementation will forward the call to [`Receiver::equal_files`] or
    /// [`Receiver::diff_files`].
    async fn lhs_and_rhs(&mut self, lhs_file_metadata: FileMetadata, rhs_file_metadata: FileMetadata) -> Result<()> {
        if lhs_file_metadata == rhs_file_metadata {
            self.equal_files(lhs_file_metadata, rhs_file_metadata).await
        } else {
            self.diff_files(lhs_file_metadata, rhs_file_metadata).await
        }
    }

    /// Receives the [`FileMetadata`] for the files that are present in both filesystems and are equal
    async fn equal_files(&mut self, _lhs_file_metadata: FileMetadata, _rhs_file_metadata: FileMetadata) -> Result<()> {
        Ok(())
    }

    /// Receives the [`FileMetadata`] for the files that are present in both filesystems and are different
    async fn diff_files(&mut self, _lhs_file_metadata: FileMetadata, _rhs_file_metadata: FileMetadata) -> Result<()> {
        Ok(())
    }
}

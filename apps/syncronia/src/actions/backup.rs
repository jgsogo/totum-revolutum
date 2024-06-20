use async_trait::async_trait;
use tracing::info;

use crate::actions::OnConflict;
use filesystem::diff::two_way_diff;
use filesystem::{FileMetadata, Filesystem, Result};

/// Backup behaviour. It can be used as a receiver in the [`two_way_diff::run`]
/// function.
pub struct Backup<'action, FsLhs: Filesystem, FsRhs: Filesystem> {
    _lhs_fs: &'action FsLhs,
    _rhs_fs: &'action FsRhs,
    on_conflict: OnConflict,
}

impl<'action, FsLhs: Filesystem, FsRhs: Filesystem> Backup<'action, FsLhs, FsRhs> {
    pub fn new(lhs_fs: &'action FsLhs, rhs_fs: &'action FsRhs, on_conflict: OnConflict) -> Self {
        Self {
            _lhs_fs: lhs_fs,
            _rhs_fs: rhs_fs,
            on_conflict,
        }
    }
}

#[async_trait]
impl<'action, FsLhs: Filesystem + 'static, FsRhs: Filesystem + 'static> two_way_diff::Receiver
    for Backup<'action, FsLhs, FsRhs>
{
    async fn only_lhs(&self, _file_metadata: Box<dyn FileMetadata>) -> Result<()> {
        todo!("not impl");
        // let relative_path = self._lhs_fs.rel_path(lhs.path())?;
        // info!("Copy to remote '{relative_path}'");
        // let rhs_file = self._rhs_fs.create()
        // Ok(())
    }

    async fn diff_files(
        &self,
        lhs_file_metadata: Box<dyn FileMetadata>,
        rhs_file_metadata: Box<dyn FileMetadata>,
    ) -> Result<()> {
        match self.on_conflict {
            OnConflict::OverrideRemote => {
                info!("Override remote '{}'", lhs_file_metadata.path());
                // let _r = remote_basepoint.copy(&lhs, Some(rhs)).await?;
            }
            OnConflict::RenameRemote => {
                info!("Rename remote and copy '{}'", rhs_file_metadata.path());
                // let _ = remote_basepoint.rename(rhs).await?;
                // let _r = remote_basepoint.copy(&lhs, None).await?;
            }
            s => panic!("Not a valid onConflict for backup: {s:?}"),
        }
        Ok(())
    }
}

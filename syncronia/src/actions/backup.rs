use camino::Utf8Path;

use async_trait::async_trait;
use tracing::info;

use crate::actions::action_run::ActionRun;
use crate::actions::OnConflict;
use filesystem::{FileMetadata, Filesystem};

pub struct Backup<'action, FsLhs: Filesystem<'action>, FsRhs: Filesystem<'action>> {
    _lhs_fs: &'action FsLhs,
    _rhs_fs: &'action FsRhs,
    on_conflict: OnConflict,
}

impl<'action, FsLhs: Filesystem<'action>, FsRhs: Filesystem<'action>> Backup<'action, FsLhs, FsRhs> {
    pub fn new(lhs_fs: &'action FsLhs, rhs_fs: &'action FsRhs, on_conflict: OnConflict) -> Self {
        Self {
            _lhs_fs: lhs_fs,
            _rhs_fs: rhs_fs,
            on_conflict,
        }
    }
}

#[async_trait]
impl<'action, FsLhs: Filesystem<'action>, FsRhs: Filesystem<'action>> ActionRun<FsLhs::Metadata, FsRhs::Metadata>
    for Backup<'action, FsLhs, FsRhs>
{
    async fn run_with_both(&self, lhs: &FsLhs::Metadata, rhs: &FsRhs::Metadata) -> anyhow::Result<()> {
        match self.on_conflict {
            OnConflict::OverrideRemote => {
                info!("Override remote '{}'", lhs.id());
                // let _r = remote_basepoint.copy(&lhs, Some(rhs)).await?;
            }
            OnConflict::RenameRemote => {
                info!("Rename remote and copy '{}'", rhs.id());
                // let _ = remote_basepoint.rename(rhs).await?;
                // let _r = remote_basepoint.copy(&lhs, None).await?;
            }
            s => panic!("Not a valid onConflict for backup: {s:?}"),
        }
        Ok(())
    }

    async fn run_with_lhs(&self, lhs: &FsLhs::Metadata) -> anyhow::Result<()> {
        let lhs_path = Utf8Path::new(lhs.id());
        let relative_path = self._lhs_fs.rel_path(lhs_path)?;
        info!("Copy to remote '{relative_path}'");
        // let rhs_file = self._rhs_fs.create()
        Ok(())
    }
}

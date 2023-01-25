use std::path::Path;

use anyhow::Result;

use super::copy::copy;
use super::Filesystem;

pub async fn move_file<'action, FsLhs: Filesystem, FsRhs: Filesystem>(
    lhs_fs: &'action FsLhs,
    rhs_fs: &'action FsRhs,
    origin: &Path,
    target: &Path,
    force: bool,
) -> Result<()> {
    copy(lhs_fs, rhs_fs, origin, target, force).await?;
    lhs_fs.remove_file(origin).await
}

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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::mocks::filesystem::FilesystemMock;

    use super::*;

    #[tokio::test]
    async fn test_move_no_force() -> Result<()> {
        todo!("Copy with NO force")
    }

    #[tokio::test]
    async fn test_move_force() -> Result<()> {
        todo!("Move with force")
    }
}

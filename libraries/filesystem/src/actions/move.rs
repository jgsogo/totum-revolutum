use camino::Utf8Path;

use crate::Filesystem;
use crate::Result;

use super::copy::copy_file;

/// Moves the content of the `origin` file in `lhs_fs` [`Filesystem`] to the `target` file
/// in the `rhs_fs` [`Filesystem`].
///
/// This action is implemented in terms of [`copy_file`].
pub async fn move_file<'action, FsLhs: Filesystem, FsRhs: Filesystem>(
    lhs_fs: &'action FsLhs,
    rhs_fs: &'action FsRhs,
    origin: &Utf8Path,
    target: &Utf8Path,
    force: bool,
) -> Result<()> {
    copy_file(lhs_fs, rhs_fs, origin, target, force).await?;
    lhs_fs.remove_file(origin).await
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

    use crate::local_temp::FilesystemLocalTemp;

    use super::*;

    #[tokio::test]
    async fn test_move_no_force() -> Result<()> {
        let file_content: Vec<u8> = b"Hello world! I'm a copy".to_vec();
        let lhs_path = Utf8PathBuf::from("file.txt");

        let lhs_fs = {
            let fs = FilesystemLocalTemp::default();
            let mut f1 = fs.create(&lhs_path).await?;
            f1.write_all(&file_content).await?;
            fs
        };

        let rhs_fs = FilesystemLocalTemp::default();
        let rhs_path = Utf8PathBuf::from("the_target.txt");
        assert!(!rhs_fs.exists(&rhs_path).await?);

        move_file(&lhs_fs, &rhs_fs, &lhs_path, &rhs_path, false).await?;
        assert!(!lhs_fs.exists(&lhs_path).await?);
        assert!(rhs_fs.exists(&rhs_path).await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_move_force() -> Result<()> {
        let file_content: Vec<u8> = b"Hello world! I'm a copy".to_vec();
        let lhs_path = Utf8PathBuf::from("file.txt");

        let lhs_fs = {
            let fs = FilesystemLocalTemp::default();
            let mut f1 = fs.create(&lhs_path).await?;
            f1.write_all(&file_content).await?;
            fs
        };

        let rhs_fs = {
            let fs = FilesystemLocalTemp::default();
            let mut f1 = fs.create(&lhs_path).await?;
            f1.write_all(&file_content).await?;
            fs
        };
        assert!(lhs_fs.exists(&lhs_path).await?);
        assert!(rhs_fs.exists(&lhs_path).await?);

        // If we don't force, we cannot move the file
        let r = move_file(&lhs_fs, &rhs_fs, &lhs_path, &lhs_path, false).await;
        assert!(r.is_err());

        // If we force, the file is moved
        let r = move_file(&lhs_fs, &rhs_fs, &lhs_path, &lhs_path, true).await;
        assert!(r.is_ok());

        assert!(!lhs_fs.exists(&lhs_path).await?);
        assert!(rhs_fs.exists(&lhs_path).await?);

        Ok(())
    }
}

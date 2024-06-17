use camino::Utf8Path;
use tokio::sync::oneshot::Receiver;

use crate::{FilesystemRead, FilesystemRemove, FilesystemWrite, Result};

use super::copy::copy_file;

/// Moves the content of the `origin` file in `lhs_fs` filesystem to the `target` file
/// in the `rhs_fs` filesystem. This action returns a [`Receiver`] that the caller can
/// use to wait for any async operation to finish.
///
/// This action is implemented in terms of [`copy_file`].
pub async fn move_file<
    'action,
    FsLhs: FilesystemRead + FilesystemRemove + ?Sized,
    FsRhs: FilesystemRead + FilesystemWrite + ?Sized,
>(
    lhs_fs: &'action mut FsLhs,
    rhs_fs: &'action mut FsRhs,
    origin: &Utf8Path,
    target: &Utf8Path,
    force: bool,
) -> Result<Option<Receiver<Result<()>>>> {
    let r = copy_file(lhs_fs, rhs_fs, origin, target, force).await?;
    lhs_fs.remove_file(origin).await?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

    use crate::impls::FilesystemLocalTemp;

    use super::*;

    async fn get_filesystem_mock_with_file(lhs_path: &Utf8Path, content: &[u8]) -> FilesystemLocalTemp {
        let mut fs = FilesystemLocalTemp::default();
        let rx = {
            let (mut f1, rx) = fs.create(&lhs_path).await.unwrap();
            f1.write_all(&content).await.unwrap();
            rx
        };
        if let Some(rx) = rx {
            rx.await.unwrap().unwrap();
        }
        fs
    }

    #[tokio::test]
    async fn test_move_no_force() -> Result<()> {
        let file_content: Vec<u8> = b"Hello world! I'm a copy".to_vec();
        let lhs_path = Utf8PathBuf::from("file.txt");

        let mut lhs_fs = get_filesystem_mock_with_file(&lhs_path, &file_content).await;

        let mut rhs_fs = FilesystemLocalTemp::default();
        let rhs_path = Utf8PathBuf::from("the_target.txt");
        assert!(!rhs_fs.exists(&rhs_path).await?);

        move_file(&mut lhs_fs, &mut rhs_fs, &lhs_path, &rhs_path, false).await?;
        assert!(!lhs_fs.exists(&lhs_path).await?);
        assert!(rhs_fs.exists(&rhs_path).await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_move_force() -> Result<()> {
        let file_content: Vec<u8> = b"Hello world! I'm a copy".to_vec();
        let lhs_path = Utf8PathBuf::from("file.txt");

        let mut lhs_fs = get_filesystem_mock_with_file(&lhs_path, &file_content).await;
        let mut rhs_fs = get_filesystem_mock_with_file(&lhs_path, &file_content).await;

        assert!(lhs_fs.exists(&lhs_path).await?);
        assert!(rhs_fs.exists(&lhs_path).await?);

        // If we don't force, we cannot move the file
        let r = move_file(&mut lhs_fs, &mut rhs_fs, &lhs_path, &lhs_path, false).await;
        assert!(r.is_err());

        // If we force, the file is moved
        let r = move_file(&mut lhs_fs, &mut rhs_fs, &lhs_path, &lhs_path, true).await;
        assert!(r.is_ok());

        assert!(!lhs_fs.exists(&lhs_path).await?);
        assert!(rhs_fs.exists(&lhs_path).await?);

        Ok(())
    }
}

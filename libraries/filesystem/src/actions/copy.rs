use camino::Utf8Path;
use tokio::sync::oneshot::Receiver;

use crate::filesystem::{FilesystemRead, FilesystemWrite};
use crate::Error;
use crate::{File, Result};

/// Copies the contents of the `lhs_file` [`File`] into the `rhs_file` [`File`]
pub async fn copy<'copy>(lhs_file: &'copy mut Box<dyn File>, rhs_file: &'copy mut Box<dyn File>) -> Result<()> {
    let mut buf: [u8; 100] = [0; 100]; // TODO: Configure buffer size. Maybe make it adaptative: https://stackoverflow.com/questions/304249/is-there-an-optimal-byte-size-for-sending-data-over-a-network
    loop {
        match lhs_file.read(&mut buf).await {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                rhs_file.write_all(&buf[..n]).await?;
            }
            Err(_) => return Err(Error::SourceFileDoesNotExist),
        }
    }
    Ok(())
}

/// Copy a file from one filesystem to another. Both instances of filesystem, the `origin` path and the
/// `target` path are provided as argument. This method returns a [`Receiver`] that the caller can use to await
/// for the operation to complete (target file is dropped and underlying filesystem has performed any async action).
pub async fn copy_file<'action, FsLhs: FilesystemRead, FsRhs: FilesystemRead + FilesystemWrite>(
    lhs_fs: &'action FsLhs,
    rhs_fs: &'action FsRhs,
    origin: &Utf8Path,
    target: &Utf8Path,
    force: bool,
) -> Result<Receiver<Result<()>>> {
    if !force && rhs_fs.exists(target).await? {
        return Err(Error::TargetFileExists);
    }

    let (mut target_file, rx) = rhs_fs.create(target).await?;
    copy(&mut lhs_fs.open(origin).await?, &mut target_file).await?;
    Ok(rx)
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

    use crate::impls::FilesystemLocalTemp;

    use super::*;

    async fn get_filesystem_mock_with_file(lhs_path: &Utf8Path, content: &[u8]) -> FilesystemLocalTemp {
        let fs = FilesystemLocalTemp::default();
        let rx = {
            let (mut f1, rx) = fs.create(&lhs_path).await.unwrap();
            f1.write_all(&content).await.unwrap();
            rx
        };
        let _ = rx.await;
        fs
    }

    #[tokio::test]
    async fn test_copy_no_force() -> Result<()> {
        let file_content: Vec<u8> = b"Hello world! I'm a copy".to_vec();
        let lhs_path = Utf8PathBuf::from("file.txt");

        let lhs_fs = get_filesystem_mock_with_file(&lhs_path, &file_content).await;
        let rhs_fs = FilesystemLocalTemp::default();
        let rhs_path = Utf8PathBuf::from("the_copy.txt");
        assert!(rhs_fs.open(&rhs_path).await.is_err());

        copy_file(&lhs_fs, &rhs_fs, &lhs_path, &rhs_path, false).await?;

        // We can read the file from the RHS
        let mut rhs_file = rhs_fs.open(&rhs_path).await?;
        let mut content_read = Vec::new();
        rhs_file.read_to_end(&mut content_read).await?;
        assert_eq!(file_content, &*content_read);

        Ok(())
    }

    #[tokio::test]
    async fn test_copy_force() -> Result<()> {
        let file_content: Vec<u8> = b"Hello world! I'm a copy".to_vec();
        let lhs_path = Utf8PathBuf::from("file.txt");

        let lhs_fs = get_filesystem_mock_with_file(&lhs_path, &file_content).await;
        let rhs_fs = get_filesystem_mock_with_file(&lhs_path, b"Any other content").await;

        // File already exists
        let r = copy_file(&lhs_fs, &rhs_fs, &lhs_path, &lhs_path, false).await;
        assert!(r.is_err());
        // ... with a different content
        let mut rhs_current_content = Vec::new();
        rhs_fs
            .open(&lhs_path)
            .await?
            .read_to_end(&mut rhs_current_content)
            .await?;
        assert_ne!(file_content, &*rhs_current_content);

        // We copy and now we get the same content
        let r = copy_file(&lhs_fs, &rhs_fs, &lhs_path, &lhs_path, true).await?;
        let _ = r.await.unwrap();

        let mut rhs_current_content = Vec::new();
        rhs_fs
            .open(&lhs_path)
            .await?
            .read_to_end(&mut rhs_current_content)
            .await?;
        assert_eq!(file_content, &*rhs_current_content);

        Ok(())
    }
}

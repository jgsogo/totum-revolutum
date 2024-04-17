use camino::Utf8Path;

use anyhow::{bail, Result};

use crate::Filesystem;

pub async fn copy<'action, FsLhs: Filesystem, FsRhs: Filesystem>(
    lhs_fs: &'action FsLhs,
    rhs_fs: &'action FsRhs,
    origin: &Utf8Path,
    target: &Utf8Path,
    force: bool,
) -> Result<()> {
    if !force && rhs_fs.exists(target).await? {
        bail!("Target file already exists. Use 'force' to override it");
    }

    let mut lhs_file = lhs_fs.open(origin).await?;
    let mut rhs_file = rhs_fs.create(target).await?;

    let mut buf: [u8; 100] = [0; 100]; // TODO: Configure buffer size
    loop {
        match lhs_file.read(&mut buf).await {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                rhs_file.write_all(&buf[..n]).await?;
            }
            Err(e) => bail!("Error reading from source: {e}"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;

    use crate::mocks::filesystem::FilesystemMock;

    use super::*;

    #[tokio::test]
    async fn test_copy_no_force() -> Result<()> {
        let file_content: Vec<u8> = b"Hello world! I'm a copy".to_vec();
        let lhs_path = Utf8PathBuf::from("file.txt");

        let lhs_fs = {
            let fs = FilesystemMock::default();
            let mut f1 = fs.create(&lhs_path).await?;
            f1.write_all(&file_content).await?;
            fs
        };

        let rhs_fs = FilesystemMock::default();
        let rhs_path = Utf8PathBuf::from("the_copy.txt");
        assert!(rhs_fs.open(&rhs_path).await.is_err());

        copy(&lhs_fs, &rhs_fs, &lhs_path, &rhs_path, false).await?;

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

        let lhs_fs = {
            let fs = FilesystemMock::default();
            let mut f1 = fs.create(&lhs_path).await?;
            f1.write_all(&file_content).await?;
            fs
        };

        let rhs_fs = {
            let fs = FilesystemMock::default();
            let mut f1 = fs.create(&lhs_path).await?;
            f1.write_all(b"Any other content").await?;
            fs
        };

        // File already exists
        let r = copy(&lhs_fs, &rhs_fs, &lhs_path, &lhs_path, false).await;
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
        let r = copy(&lhs_fs, &rhs_fs, &lhs_path, &lhs_path, true).await;
        assert!(r.is_ok());
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

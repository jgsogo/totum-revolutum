use std::path::Path;

use anyhow::{bail, Result};

use super::Filesystem;

pub async fn copy<'action, FsLhs: Filesystem, FsRhs: Filesystem>(
    lhs_fs: &'action FsLhs,
    rhs_fs: &'action FsRhs,
    origin: &Path,
    target: &Path,
    _force: bool,
) -> Result<()> {
    let mut lhs_file = lhs_fs.open(origin).await?;
    // TODO: Depending on `force` value, check if the file already exists
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
    use std::path::PathBuf;

    use crate::mocks::filesystem::FilesystemMock;

    use super::*;

    #[tokio::test]
    async fn test_copy() -> Result<()> {
        let file_content: Vec<u8> = b"Hello world! I'm a copy".to_vec();
        let lhs_path = PathBuf::from("file.txt");

        let lhs_fs = {
            let fs = FilesystemMock::default();
            let mut f1 = fs.create(&lhs_path).await?;
            f1.write_all(&file_content).await?;
            fs
        };

        let rhs_fs = FilesystemMock::default();
        let rhs_path = PathBuf::from("the_copy.txt");
        assert!(rhs_fs.open(&rhs_path).await.is_err());

        copy(&lhs_fs, &rhs_fs, &lhs_path, &rhs_path, false).await?;

        // We can read the file from the RHS
        let mut rhs_file = rhs_fs.open(&rhs_path).await?;
        let mut content_read = Vec::new();
        rhs_file.read_to_end(&mut content_read).await?;
        assert_eq!(file_content, &*content_read);

        Ok(())
    }
}

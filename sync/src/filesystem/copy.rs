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

    let mut buf: [u8; 100] = [0; 100];
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

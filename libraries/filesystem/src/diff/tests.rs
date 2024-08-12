//! Utilities for testing

use std::str::FromStr;

use crate::impls::FilesystemLocalTemp;
use crate::wrappers::AsyncFileDropImpl;
use crate::{DirectoryPathBuf, Error, FilePath, FilePathBuf, FilenameBuf, Filesystem, Result};

/// Some data to be used for testing
pub(crate) struct DiffMocks {
    pub(crate) lhs_only: FilePathBuf,
    pub(crate) rhs_only: FilePathBuf,
    pub(crate) both_equal: FilePathBuf,
    pub(crate) diff_hash: FilePathBuf,
    pub(crate) diff_size: FilePathBuf,

    pub(crate) fs_lhs: AsyncFileDropImpl<FilesystemLocalTemp>,
    pub(crate) fs_rhs: AsyncFileDropImpl<FilesystemLocalTemp>,
}

impl DiffMocks {
    pub(crate) async fn new() -> Result<Self> {
        let root = DirectoryPathBuf::root();
        let lhs_only = root.join_filename(FilenameBuf::from_str("lhs_only")?);
        let rhs_only = root.join_filename(FilenameBuf::from_str("rhs_only")?);
        let both_equal = root.join_filename(FilenameBuf::from_str("both_equal")?);
        let diff_hash = root.join_filename(FilenameBuf::from_str("diff_hash")?);
        let diff_size = root.join_filename(FilenameBuf::from_str("diff_size")?);

        let fs_lhs = {
            let mut fs = AsyncFileDropImpl::new_call_sync_all(FilesystemLocalTemp::default());
            create_file(&mut fs, &lhs_only, b"anything").await?;
            create_file(&mut fs, &both_equal, b"both_equal").await?;
            create_file(&mut fs, &diff_hash, b"lhs_hash").await?;
            create_file(&mut fs, &diff_size, b"lhs size").await?;
            fs
        };

        let fs_rhs = {
            let mut fs = AsyncFileDropImpl::new_call_sync_all(FilesystemLocalTemp::default());
            create_file(&mut fs, &rhs_only, b"anything").await?;
            create_file(&mut fs, &both_equal, b"both_equal").await?;
            create_file(&mut fs, &diff_hash, b"rhs_hash").await?;
            create_file(&mut fs, &diff_size, b"rhs size so it's different").await?;
            fs
        };

        Ok(Self {
            lhs_only,
            rhs_only,
            both_equal,
            diff_hash,
            diff_size,
            fs_lhs,
            fs_rhs,
        })
    }
}

async fn create_file(filesystem: &mut dyn Filesystem, path: &FilePath, content: &[u8]) -> crate::Result<()> {
    let rx = {
        let (mut f, rx) = filesystem.create(path).await?;
        f.write_all(content).await?;
        rx.unwrap()
    };

    rx.await.map_err(|e| Error::Other(e.to_string()))??;
    Ok(())
}

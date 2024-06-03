use std::fs::File;
use std::io::Read;

use async_trait::async_trait;
use camino::Utf8Path;

use crate::methods::fileops::file_close::GetFileClose;
use crate::methods::fileops::file_open::{FileOpenPath, Flags, GetFileOpen};
use crate::methods::fileops::file_write::PostFileWrite;
use crate::types::FileID;
use crate::Result;

#[async_trait]
/// Uploads a local file to the given [`FileID`]
pub trait UploadToFileID {
    async fn upload_to_fileid(&self, local_path: &Utf8Path, fileid: FileID) -> Result<()>;
}

#[async_trait]
impl<T: GetFileOpen + PostFileWrite + GetFileClose + Sync> UploadToFileID for T {
    async fn upload_to_fileid(&self, local_path: &Utf8Path, fileid: FileID) -> Result<()> {
        let fd = self
            .file_open(Flags::O_WRITE | Flags::O_TRUNC, FileOpenPath::File(fileid.into()))
            .await?;

        let mut buffer = Vec::new();
        let bytes_read = File::open(local_path)?.read_to_end(&mut buffer)?;

        let bytes_count = self.file_write(fd.fd.clone(), &buffer).await?;
        assert_eq!(bytes_count.bytes, bytes_read as u64);
        self.file_close(fd.fd).await?;

        Ok(())
    }
}

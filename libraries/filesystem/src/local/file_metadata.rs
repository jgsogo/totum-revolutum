use camino::{Utf8Path, Utf8PathBuf};
use ignore::DirEntry;

use crate::FileMetadata;

#[derive(Debug, Clone)]
pub struct LocalMetadata {
    relative_path: Utf8PathBuf,
    entry: DirEntry,
}

impl FileMetadata for LocalMetadata {
    fn path(&self) -> &Utf8Path {
        &self.relative_path
    }

    fn size(&self) -> u64 {
        self.entry.metadata().unwrap().len()
    }

    fn hash(&self) -> String {
        sha256::try_digest(self.entry.path()).unwrap()
    }
}

impl LocalMetadata {
    pub fn new(relative_path: Utf8PathBuf, entry: DirEntry) -> Self {
        Self { relative_path, entry }
    }
}

// #[async_trait]
// impl Copy<LocalMetadata> for LocalMetadata {
//     async fn copy(self, rhs: Option<LocalMetadata>) -> Result<(Self, LocalMetadata)> {
//         trace!(
//             "Copy local '{}' to local '{}'",
//             self.path().display(),
//             rhs.as_ref().map_or("".to_string(), |v| v.path().display().to_string())
//         );
//         // std::fs::copy(self.path(), rhs.path())
//         //     .map_err(|e| anyhow!("Error copying file from '{}' to '{}'", self.path(), rhs.path()))?;
//         Ok((self, rhs.unwrap()))
//     }
// }

// #[async_trait]
// impl Remove for LocalMetadata {
//     async fn remove(self) -> Result<()> {
//         trace!("Remove local '{}'", self.path().display());
//         Ok(())
//     }
// }
//
// #[async_trait]
// impl Rename for LocalMetadata {
//     async fn rename(self) -> Result<Self> {
//         trace!("Rename local '{}'", self.path().display());
//         Ok(self)
//     }
// }

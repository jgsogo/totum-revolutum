use std::path::Path;

use ignore::DirEntry;

use crate::diff::filesystem::FileMetadata;

pub trait LocalFileMetadata: FileMetadata {
    fn from_direntry(base_path: &Path, entry: DirEntry) -> Self;

    fn path(&self) -> &Path;
}

#[derive(Debug, Clone)]
pub struct LocalMetadata {
    id: String,
    entry: DirEntry,
}

impl FileMetadata for LocalMetadata {
    fn id(&self) -> &str {
        &self.id
    }
    fn size(&self) -> u64 {
        self.entry.metadata().unwrap().len()
    }

    fn hash(&self) -> String {
        sha256::try_digest(self.path()).unwrap()
    }
}

impl LocalFileMetadata for LocalMetadata {
    fn from_direntry(base_path: &Path, entry: DirEntry) -> Self {
        Self {
            id: entry
                .path()
                .strip_prefix(base_path)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            entry,
        }
    }

    fn path(&self) -> &Path {
        self.entry.path()
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

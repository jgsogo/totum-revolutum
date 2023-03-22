use serde::{Deserialize, Serialize};

use crate::types::FolderID;

use super::{CommonMetadata, Metadata};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MetadataFolder {
    // Given `filtermeta` argument, everything is optional. However, we make this field required so
    // the parser can differentiate between this [`MetadataFile`] and [`MetadataFolder`].
    pub folderid: FolderID,

    #[serde(flatten)]
    pub common: CommonMetadata,

    pub contents: Option<Vec<Metadata>>,

    // only for folders
    pub cancreate: Option<bool>,
}

impl MetadataFolder {
    #[cfg(feature = "test_utils")]
    pub fn default(folderid: FolderID) -> Self {
        Self {
            folderid,
            common: CommonMetadata::default(),
            contents: None,
            cancreate: None,
        }
    }
}

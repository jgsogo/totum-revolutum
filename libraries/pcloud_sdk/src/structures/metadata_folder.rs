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

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    use camino::Utf8Path;
    use time::macros::datetime;

    use crate::types::FolderID;

    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_deserialize_metadata_folder() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("metadata_folder.json");
        let file = File::open(manifest_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, MetadataFolder>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                // TODO: Check that all fields are here
                assert_eq!(data.common.isfolder, Some(true));

                assert_eq!(data.common.ismine, Some(true));

                assert_eq!(data.common.isshared, Some(false));
                assert_eq!(data.common.name, Some("a folder".to_string()));
                assert_eq!(data.common.id, Some("d1729212".to_string()));
                assert_eq!(data.folderid, FolderID(1729212));
                assert_eq!(data.common.created, Some(datetime!(2013-10-02 14:29:11 UTC)));
                assert_eq!(data.common.modified, None);
                assert_eq!(data.common.isdeleted, None);
            }
        }
    }
}

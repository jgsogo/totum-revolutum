use serde::{Deserialize, Serialize};

use super::{MetadataFile, MetadataFolder};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Metadata {
    MetadataFile(MetadataFile),
    MetadataFolder(MetadataFolder),
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    use camino::Utf8Path;
    use time::macros::datetime;

    use crate::types::{FileID, FolderID};

    use super::super::category::Category;
    use super::super::icon::Icon;
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_deserialize_metadata_file() {
        // A manifest for a file, cannot be read as a [`MetadataFolder`]
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("metadata_file.json");
        let file = File::open(manifest_json).unwrap();
        let reader = BufReader::new(file);
        assert!(serde_json::from_reader::<_, MetadataFolder>(reader).is_err());

        // it can be read as a [`MetadataFile`]
        let manifest_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("metadata_file.json");
        let file = File::open(manifest_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, MetadataFile>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                // TODO: Check that all fields are here
                assert_eq!(data.common.isfolder, Some(false));

                assert_eq!(data.common.ismine, Some(true));

                assert_eq!(data.common.isshared, Some(false));
                assert_eq!(data.common.name, Some("Simple image.jpg".to_string()));
                assert_eq!(data.common.id, Some("f1729212".to_string()));
                assert_eq!(data.fileid, FileID(1729212));
                assert_eq!(data.deletedfileid, None);
                assert_eq!(data.common.created, Some(datetime!(2013-10-02 14:29:11 UTC)));
                assert_eq!(data.common.modified, None);
                assert_eq!(data.common.icon, Some(Icon::Image));
                assert_eq!(data.category, Some(Category::Image));
                assert_eq!(data.common.thumb, Some(true));
                assert_eq!(data.size, Some(73269));
                assert_eq!(data.contenttype, Some("image/jpeg".into()));
                assert_eq!(data.hash, Some(10681749967730527559));
                assert_eq!(data.common.isdeleted, None);
                assert_eq!(data.common.path, Some("/Simple image.jpg".into()));
            }
        }
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_deserialize_metadata_folder() {
        // A manifest for a folder, cannot be read as a [`MetadataFile`]
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("metadata_folder.json");
        let file = File::open(manifest_json).unwrap();
        let reader = BufReader::new(file);
        assert!(serde_json::from_reader::<_, MetadataFile>(reader).is_err());

        // it can be read as a [`MetadataFolder`]
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

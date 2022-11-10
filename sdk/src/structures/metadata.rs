use super::category::Category;
use crate::id::{FileID, FolderID};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// TODO: Use enum for files and folders: https://serde.rs/enum-representations.html

// https://docs.pcloud.com/structures/metadata.html
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
/// Metadata that is common to files and folders
pub struct CommonMetadata {
    icon: String,
    id: String,
    #[serde(with = "time::serde::rfc2822")]
    created: OffsetDateTime,
    #[serde(with = "time::serde::rfc2822::option")]
    modified: Option<OffsetDateTime>,
    path: Option<String>,
    thumb: bool,
    isfolder: bool,
    isshared: bool,
    ismine: bool,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Metadata {
    #[serde(flatten)]
    common: CommonMetadata,

    parentfolderid: Option<FolderID>,
    canread: Option<bool>,
    canmodify: Option<bool>,
    candelete: Option<bool>,
    cancreate: Option<bool>,

    pub folderid: Option<FolderID>,
    fileid: Option<FileID>,
    deletedfileid: Option<FileID>,
    category: Option<Category>,
    size: Option<i64>,
    contenttype: Option<String>,
    hash: Option<u64>,
    contents: Option<Vec<Metadata>>,
    isdeleted: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use time::macros::datetime;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_deserialize() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_json = Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("metadata_file.json");
        let file = File::open(manifest_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, Metadata>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.parentfolderid, Some(FolderID(0)));
                assert_eq!(data.common.isfolder, false);

                assert_eq!(data.common.ismine, true);
                assert_eq!(data.canread, None);
                assert_eq!(data.canmodify, None);
                assert_eq!(data.candelete, None);
                assert_eq!(data.cancreate, None);

                assert_eq!(data.common.isshared, false);
                assert_eq!(data.common.name, "Simple image.jpg");
                assert_eq!(data.common.id, "f1729212");
                assert_eq!(data.folderid, None);
                assert_eq!(data.fileid, Some(FileID(1729212)));
                assert_eq!(data.deletedfileid, None);
                assert_eq!(data.common.created, datetime!(2013-10-02 14:29:11 UTC));
                assert_eq!(
                    data.common.modified,
                    Some(datetime!(2013-10-02 14:29:11 UTC))
                );
                assert_eq!(data.common.icon, "image");
                assert_eq!(data.category, Some(Category::Image));
                assert_eq!(data.common.thumb, true);
                assert_eq!(data.size, Some(73269));
                assert_eq!(data.contenttype, Some("image/jpeg".into()));
                assert_eq!(data.hash, Some(10681749967730527559));
                assert_eq!(data.contents, None);
                assert_eq!(data.isdeleted, None);
                assert_eq!(data.common.path, Some("/Simple image.jpg".into()));
            }
        }
    }
}

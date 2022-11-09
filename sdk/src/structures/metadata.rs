use crate::id::{FileID, FolderID};
use serde::{Deserialize, Serialize};
type Timestamp = String;

// https://docs.pcloud.com/structures/metadata.html
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Metadata {
    parentfolderid: Option<FolderID>,
    isfolder: bool,

    ismine: bool,
    canread: Option<bool>,
    canmodify: Option<bool>,
    candelete: Option<bool>,
    cancreate: Option<bool>,

    isshared: bool,
    name: String,
    id: String,
    pub folderid: Option<FolderID>,
    fileid: Option<FileID>,
    deletedfileid: Option<FileID>,
    created: Timestamp,
    modified: Option<Timestamp>,
    icon: String,
    category: Option<u8>,
    // This is an enumerated type
    thumb: bool,
    size: Option<i64>,
    contenttype: Option<String>,
    hash: Option<u64>,
    contents: Option<Vec<Metadata>>,
    isdeleted: Option<bool>,
    path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[test]
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
                assert_eq!(data.isfolder, false);

                assert_eq!(data.ismine, true);
                assert_eq!(data.canread, None);
                assert_eq!(data.canmodify, None);
                assert_eq!(data.candelete, None);
                assert_eq!(data.cancreate, None);

                assert_eq!(data.isshared, false);
                assert_eq!(data.name, "Simple image.jpg");
                assert_eq!(data.id, "f1729212");
                assert_eq!(data.folderid, None);
                assert_eq!(data.fileid, Some(FileID(1729212)));
                assert_eq!(data.deletedfileid, None);
                assert_eq!(data.created, "Wed, 02 Oct 2013 14:29:11 +0000");
                assert_eq!(
                    data.modified,
                    Some("Wed, 02 Oct 2013 14:29:11 +0000".into())
                );
                assert_eq!(data.icon, "image");
                assert_eq!(data.category, Some(1));
                assert_eq!(data.thumb, true);
                assert_eq!(data.size, Some(73269));
                assert_eq!(data.contenttype, Some("image/jpeg".into()));
                assert_eq!(data.hash, Some(10681749967730527559));
                assert_eq!(data.contents, None);
                assert_eq!(data.isdeleted, None);
                assert_eq!(data.path, Some("/Simple image.jpg".into()));
            }
        }
    }
}

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::types::{FileID, FolderID};

use super::category::Category;
use super::icon::Icon;

// TODO: Use enum for files and folders: https://serde.rs/enum-representations.html

// https://docs.pcloud.com/structures/metadata.html
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
/// Metadata that is common to files and folders
///
/// Given `filtermeta` argument, everything is optional
pub struct CommonMetadata {
    icon: Option<Icon>,
    id: Option<String>,
    #[serde(with = "time::serde::rfc2822::option", default)]
    created: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc2822::option", default)]
    modified: Option<OffsetDateTime>,
    pub path: Option<String>,
    thumb: Option<bool>,
    pub isfolder: Option<bool>,
    isshared: Option<bool>,
    ismine: Option<bool>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Metadata {
    #[serde(flatten)]
    pub common: CommonMetadata,
    parentfolderid: Option<FolderID>,

    canread: Option<bool>,
    canmodify: Option<bool>,
    candelete: Option<bool>,

    pub folderid: Option<FolderID>,
    pub fileid: Option<FileID>,
    deletedfileid: Option<FileID>,
    category: Option<Category>,
    pub contents: Option<Vec<Metadata>>,
    isdeleted: Option<bool>,

    // only for folders
    cancreate: Option<bool>,

    // only for files
    pub hash: Option<u64>,
    pub size: Option<u64>,
    contenttype: Option<String>,

    // Optional fields depending on file type
    #[serde(flatten)]
    extra_imagefile: Option<MetadataImageFile>,
    #[serde(flatten)]
    extra_audiofile: Option<MetadataAudioFile>,
    #[serde(flatten)]
    extra_videofile: Option<MetadataVideoFile>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MetadataImageFile {
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MetadataAudioFile {
    artist: String,
    album: u32,
    title: u32,
    genre: u32,
    trackno: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MetadataVideoFile {
    width: String,
    height: u32,
    duration: f32,
    fps: f32,
    videocodec: String,
    audiocodec: String,
    videobitrate: u32,
    audiobitrate: u32,
    audiosamplerate: u32,
    rotate: u16,
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use time::macros::datetime;

    use super::*;

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
                assert_eq!(data.common.isfolder, Some(false));

                assert_eq!(data.common.ismine, Some(true));
                assert_eq!(data.canread, None);
                assert_eq!(data.canmodify, None);
                assert_eq!(data.candelete, None);
                assert_eq!(data.cancreate, None);

                assert_eq!(data.common.isshared, Some(false));
                assert_eq!(data.common.name, Some("Simple image.jpg".to_string()));
                assert_eq!(data.common.id, Some("f1729212".to_string()));
                assert_eq!(data.folderid, None);
                assert_eq!(data.fileid, Some(FileID(1729212)));
                assert_eq!(data.deletedfileid, None);
                assert_eq!(data.common.created, Some(datetime!(2013-10-02 14:29:11 UTC)));
                assert_eq!(data.common.modified, None);
                assert_eq!(data.common.icon, Some(Icon::Image));
                assert_eq!(data.category, Some(Category::Image));
                assert_eq!(data.common.thumb, Some(true));
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

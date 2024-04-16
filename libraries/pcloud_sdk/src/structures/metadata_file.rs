use serde::{Deserialize, Serialize};

use crate::types::FileID;

use super::category::Category;
use super::CommonMetadata;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MetadataFile {
    // Given `filtermeta` argument, everything is optional. However, we make this field required so
    // the parser can differentiate between this [`MetadataFile`] and [`MetadataFolder`].
    pub fileid: FileID,

    #[serde(flatten)]
    pub common: CommonMetadata,

    pub deletedfileid: Option<FileID>,
    pub category: Option<Category>,

    // only for files
    pub hash: Option<u64>,
    pub size: Option<u64>,
    pub contenttype: Option<String>,

    // Optional fields depending on file type
    #[serde(flatten)]
    pub extra_imagefile: Option<MetadataImageFile>,
    #[serde(flatten)]
    pub extra_audiofile: Option<MetadataAudioFile>,
    #[serde(flatten)]
    pub extra_videofile: Option<MetadataVideoFile>,
}

impl MetadataFile {
    #[cfg(feature = "test_utils")]
    pub fn default(fileid: FileID) -> Self {
        Self {
            fileid,
            common: CommonMetadata::default(),
            deletedfileid: None,
            category: None,
            hash: None,
            size: None,
            contenttype: None,
            extra_imagefile: None,
            extra_audiofile: None,
            extra_videofile: None,
        }
    }
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

    use camino::Utf8Path;
    use time::macros::datetime;

    use crate::types::FileID;

    use super::super::category::Category;
    use super::super::icon::Icon;
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_deserialize_metadata_file() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
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
}

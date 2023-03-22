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

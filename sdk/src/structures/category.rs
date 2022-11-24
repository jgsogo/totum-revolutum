use std::fmt::{Debug, Display, Formatter};

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
/// Category of the file, described in https://docs.pcloud.com/structures/metadata.html
pub enum Category {
    Uncategorized = 0,
    Image = 1,
    Video = 2,
    Audio = 3,
    Document = 4,
    Archive = 5,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let readable = match self {
            Category::Uncategorized => "uncategorized",
            Category::Image => "image",
            Category::Video => "video",
            Category::Audio => "audio",
            Category::Document => "document",
            Category::Archive => "archive",
        };
        write!(f, "{}", readable)
    }
}

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let readable = match self {
            Category::Uncategorized => "uncategorized(0)",
            Category::Image => "image(1)",
            Category::Video => "video(2)",
            Category::Audio => "audio(3)",
            Category::Document => "document(4)",
            Category::Archive => "archive(5)",
        };
        write!(f, "{}", readable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Category::Uncategorized.to_string(), "uncategorized");
        assert_eq!(Category::Image.to_string(), "image");
        assert_eq!(Category::Video.to_string(), "video");
        assert_eq!(Category::Audio.to_string(), "audio");
        assert_eq!(Category::Document.to_string(), "document");
        assert_eq!(Category::Archive.to_string(), "archive");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Category::Uncategorized), "uncategorized(0)");
        assert_eq!(format!("{:?}", Category::Image), "image(1)");
        assert_eq!(format!("{:?}", Category::Video), "video(2)");
        assert_eq!(format!("{:?}", Category::Audio), "audio(3)");
        assert_eq!(format!("{:?}", Category::Document), "document(4)");
        assert_eq!(format!("{:?}", Category::Archive), "archive(5)");
    }

    #[test]
    fn test_serialize() {
        assert_eq!(
            serde_json::to_string(&Category::Uncategorized).unwrap(),
            "0"
        );
        assert_eq!(serde_json::to_string(&Category::Image).unwrap(), "1");
        assert_eq!(serde_json::to_string(&Category::Video).unwrap(), "2");
        assert_eq!(serde_json::to_string(&Category::Audio).unwrap(), "3");
        assert_eq!(serde_json::to_string(&Category::Document).unwrap(), "4");
        assert_eq!(serde_json::to_string(&Category::Archive).unwrap(), "5");
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            serde_json::from_str::<Category>("0").unwrap(),
            Category::Uncategorized
        );
        assert_eq!(
            serde_json::from_str::<Category>("1").unwrap(),
            Category::Image
        );
        assert_eq!(
            serde_json::from_str::<Category>("2").unwrap(),
            Category::Video
        );
        assert_eq!(
            serde_json::from_str::<Category>("3").unwrap(),
            Category::Audio
        );
        assert_eq!(
            serde_json::from_str::<Category>("4").unwrap(),
            Category::Document
        );
        assert_eq!(
            serde_json::from_str::<Category>("5").unwrap(),
            Category::Archive
        );
    }
}

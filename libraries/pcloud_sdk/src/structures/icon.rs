use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// Name of the icon to display (one of document, database, archive, web, gis,
/// spreadsheet, font, presentation, image, diskimage, package, executable,
/// audio, video, file
///
/// Described in https://docs.pcloud.com/structures/metadata.html
#[derive(Serialize, Deserialize, PartialEq, Eq, Display, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[allow(clippy::upper_case_acronyms)]
#[strum(serialize_all = "snake_case")]
pub enum Icon {
    Document,
    Database,
    Archive,
    Web,
    GIS,
    Spreadsheet,
    Font,
    Presentation,
    Image,
    DiskImage,
    Package,
    Executable,
    Audio,
    Video,
    File,
    #[serde(other)]
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Icon::Document.to_string(), "document");
        assert_eq!(Icon::GIS.to_string(), "gis");
        assert_eq!(Icon::DiskImage.to_string(), "disk_image");
        assert_eq!(Icon::Other.to_string(), "other");
    }

    // #[test]
    // fn test_debug() {
    //     assert_eq!(format!("{:?}", Icon::Document), "uncategorized(0)");
    //     assert_eq!(format!("{:?}", Icon::GIS), "image(1)");
    //     assert_eq!(format!("{:?}", Icon::DiskImage), "video(2)");
    //     assert_eq!(
    //         format!("{:?}", Icon::Other("OtherThing".into())),
    //         "video(2)"
    //     );
    // }

    #[test]
    fn test_serialize() {
        assert_eq!(serde_json::to_string(&Icon::Document).unwrap(), "\"document\"");
        assert_eq!(serde_json::to_string(&Icon::GIS).unwrap(), "\"gis\"");
        assert_eq!(serde_json::to_string(&Icon::DiskImage).unwrap(), "\"diskimage\"");
        assert_eq!(serde_json::to_string(&Icon::Other).unwrap(), "\"other\"");
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(serde_json::from_str::<Icon>("\"document\"").unwrap(), Icon::Document);
        assert_eq!(serde_json::from_str::<Icon>("\"gis\"").unwrap(), Icon::GIS);
        assert_eq!(serde_json::from_str::<Icon>("\"diskimage\"").unwrap(), Icon::DiskImage);
        assert_eq!(serde_json::from_str::<Icon>("\"otherthing\"").unwrap(), Icon::Other);
    }
}

use serde::{Deserialize, Serialize};

use super::{CommonMetadata, MetadataFile, MetadataFolder};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Metadata {
    MetadataFile(MetadataFile),
    MetadataFolder(MetadataFolder),
}

impl Metadata {
    /// Return the [`CommonMetadata`] chunk, which is available in all enum variants.
    pub fn common(&self) -> &CommonMetadata {
        match self {
            Metadata::MetadataFile(mfile) => &mfile.common,
            Metadata::MetadataFolder(mfolder) => &mfolder.common,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    use camino::Utf8Path;

    use super::*;

    #[test]
    fn test_deserialize_metadata_file() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("metadata_file.json");

        // A manifest for a file, cannot be read as a [`MetadataFolder`]
        let file = File::open(&manifest_json).unwrap();
        let reader = BufReader::new(file);
        assert!(serde_json::from_reader::<_, MetadataFolder>(reader).is_err());

        // it can be read as a [`MetadataFile`]
        let file = File::open(&manifest_json).unwrap();
        let reader = BufReader::new(file);
        assert!(serde_json::from_reader::<_, MetadataFile>(reader).is_ok());

        // it can be read as a [`Metadata`] -> returns [`MetadataFile`]
        let file = File::open(&manifest_json).unwrap();
        let reader = BufReader::new(file);
        let r = serde_json::from_reader::<_, Metadata>(reader);
        assert!(r.is_ok());
        assert!(matches!(r.unwrap(), Metadata::MetadataFile { .. }));
    }

    #[test]
    fn test_deserialize_metadata_folder() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("metadata_folder.json");

        // A manifest for a folder, cannot be read as a [`MetadataFile`]
        let file = File::open(&manifest_json).unwrap();
        let reader = BufReader::new(file);
        assert!(serde_json::from_reader::<_, MetadataFile>(reader).is_err());

        // it can be read as a [`MetadataFolder`]
        let file = File::open(&manifest_json).unwrap();
        let reader = BufReader::new(file);
        assert!(serde_json::from_reader::<_, MetadataFolder>(reader).is_ok());

        // it can be read as a [`Metadata`] -> returns [`MetadataFolder`]
        let file = File::open(&manifest_json).unwrap();
        let reader = BufReader::new(file);
        let r = serde_json::from_reader::<_, Metadata>(reader);
        assert!(r.is_ok());
        assert!(matches!(r.unwrap(), Metadata::MetadataFolder { .. }));
    }
}

use serde::{Deserialize, Serialize};

type Timestamp = String;

// https://docs.pcloud.com/structures/metadata.html
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    parentfolderid: Option<i64>,
    isfolder: bool,

    ismine: bool,
    canread: Option<bool>,
    canmodify: Option<bool>,
    candelete: Option<bool>,
    cancreate: Option<bool>,

    isshared: bool,
    name: String,
    id: String,
    pub folderid: Option<i64>,
    fileid: Option<i64>,
    deletedfileid: Option<i64>,
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
        let manifest_json = Path::new(&manifest_dir).join("resources").join("testdata").join("metadata.json");

        let file = File::open(manifest_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, Metadata>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                
            }
        }

        assert_eq!(manifest_dir, "lol");
        // match serde_json::from_str(&result) {
        //     Ok(data) => Ok(data),
        //     Err(e) => {
        //         // TODO: Provide enough information to debug, but also return meaningful error
        //         Err(Box::new(Error::SerializationError(e)) as Box<dyn std::error::Error + Send + Sync>)
        //         /*
        //         Err(Box::new(Error::APIError(&format!(
        //             "Cannot parse '{}' into {}",
        //             result,
        //             std::any::type_name::<T>()
        //         ))))
        //         */
        //     }
        // }

        // let fileid= FileID(23);
        // assert_eq!(fileid.to_string(), "fileid:23");
        // assert_eq!(format!("{fileid:?}"), "fileid:23");
    }

}


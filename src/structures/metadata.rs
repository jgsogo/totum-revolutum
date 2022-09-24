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

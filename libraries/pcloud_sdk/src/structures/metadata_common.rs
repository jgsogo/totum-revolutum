use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::types::FolderID;

use super::icon::Icon;

// https://docs.pcloud.com/structures/metadata.html
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "test_utils", derive(Default))]
/// Metadata that is common to files and folders
///
/// Given `filtermeta` argument, everything is optional
pub struct CommonMetadata {
    pub(crate) icon: Option<Icon>,
    pub(crate) id: Option<String>,
    #[serde(with = "time::serde::rfc2822::option", default)]
    pub(crate) created: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc2822::option", default)]
    pub(crate) modified: Option<OffsetDateTime>,
    pub path: Option<String>,
    pub(crate) thumb: Option<bool>,
    pub isfolder: Option<bool>,
    pub(crate) isshared: Option<bool>,

    pub(crate) ismine: Option<bool>,
    pub canread: Option<bool>,
    pub canmodify: Option<bool>,
    pub candelete: Option<bool>,

    pub name: Option<String>,
    pub isdeleted: Option<bool>,

    pub parentfolderid: Option<FolderID>,
}

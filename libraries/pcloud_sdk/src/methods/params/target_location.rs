use std::collections::HashMap;

use http_utils::AddToParams;

use crate::types::{FolderID, RemotePath};

/// Provides a target location identification, using either
///
///  * a [`FolderID`] and a target name (optional)
///  * a path
///
/// In both cases, if the name of the target is not provided or the path ends in `/`, it
/// will use original target name
pub enum TargetLocation {
    FolderAndName((FolderID, Option<String>)),
    RemotePath(RemotePath),
}

impl AddToParams for TargetLocation {
    fn add_to_params(&self, params: &mut HashMap<String, String>) {
        match self {
            TargetLocation::FolderAndName((folderid, name)) => {
                params.insert("tofolderid".to_string(), folderid.inner().to_string());
                if let Some(name) = name {
                    params.insert("toname".to_string(), name.clone());
                }
            }
            TargetLocation::RemotePath(p) => {
                params.insert("topath".to_string(), p.as_path().to_string());
            }
        }
    }
}

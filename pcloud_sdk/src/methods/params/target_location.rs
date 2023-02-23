use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;

use crate::types::FolderID;

use super::Params;

/// Provides a target location identification, using either
///
///  * a [`FolderID`] and a target name (optional)
///  * a path
///
/// In both cases, if the name of the target is not provided or the path ends in `/`, it
/// will use original target name
pub enum TargetLocation {
    FolderAndName((FolderID, Option<String>)),
    Path(PathBuf),
}

impl Params for TargetLocation {
    fn add_to_params(&self, params: &mut HashMap<String, String>) -> Result<()> {
        match self {
            TargetLocation::FolderAndName((folderid, name)) => {
                params.insert("tofolderid".to_string(), folderid.0.to_string());
                if let Some(name) = name {
                    params.insert("toname".to_string(), name.clone());
                }
            }
            TargetLocation::Path(p) => {
                params.insert("topath".to_string(), p.to_string_lossy().parse()?);
            }
        }
        Ok(())
    }
}

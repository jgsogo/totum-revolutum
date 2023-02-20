use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;

use crate::types::FolderID;

use super::Params;

/// Provides a target file identification, using either
///
///  * a [`FolderID`] and a file name (optional)
///  * a path
///
/// In both cases, if the name of the file is not provided or the path ends in `/`, it
/// will use original filename
pub enum TargetFile {
    FolderAndName((FolderID, Option<String>)),
    Path(PathBuf),
}

impl Params for TargetFile {
    fn add_to_params(&self, params: &mut HashMap<String, String>) -> Result<()> {
        match self {
            TargetFile::FolderAndName((folderid, name)) => {
                params.insert("tofolderid".to_string(), folderid.0.to_string());
                if let Some(name) = name {
                    params.insert("toname".to_string(), name.clone());
                }
            }
            TargetFile::Path(p) => {
                params.insert("topath".to_string(), p.to_string_lossy().parse()?);
            }
        }
        Ok(())
    }
}

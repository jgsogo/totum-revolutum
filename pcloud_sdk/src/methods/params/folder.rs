use std::collections::HashMap;

use anyhow::Result;

use crate::types::Folder;

use super::Params;

impl Params for Folder {
    fn add_to_params(&self, params: &mut HashMap<String, String>) -> Result<()> {
        match self {
            Folder::FolderID(fid) => {
                params.insert("folderid".to_string(), fid.0.to_string());
            }
            Folder::RemotePath(p) => {
                params.insert("path".to_string(), p.path().to_string());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::FolderID;

    use super::*;

    #[test]
    fn test_params_with_fileid() -> Result<()> {
        let input = Folder::FolderID(FolderID(1234));
        let mut params = HashMap::new();
        input.add_to_params(&mut params)?;
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
        Ok(())
    }

    #[test]
    fn test_params_with_path() -> Result<()> {
        let input = Folder::from_str("path:/this/is/the/path").unwrap();
        let mut params = HashMap::new();
        input.add_to_params(&mut params)?;
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("path"), Some(&"/this/is/the/path".to_string()));
        Ok(())
    }
}

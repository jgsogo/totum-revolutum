use std::collections::HashMap;

use crate::types::Folder;

impl TryFrom<Folder> for HashMap<String, String> {
    type Error = anyhow::Error;

    fn try_from(value: Folder) -> std::result::Result<Self, Self::Error> {
        let mut params = HashMap::new();
        match value {
            Folder::FolderID(fid) => {
                params.insert("folderid".to_string(), fid.0.to_string());
            }
            Folder::Path(p) => {
                params.insert("path".to_string(), p.to_string_lossy().parse()?);
            }
        };
        Ok(params)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::FolderID;

    use super::*;

    #[test]
    fn test_params_with_fileid() {
        let input = Folder::FolderID(FolderID(1234));
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
    }

    #[test]
    fn test_params_with_path() {
        let input = Folder::from_str("/this/is/the/path").unwrap();
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("path"), Some(&"/this/is/the/path".to_string()));
    }
}

use std::collections::HashMap;

use crate::types::File;

impl TryFrom<File> for HashMap<String, String> {
    type Error = anyhow::Error;

    fn try_from(value: File) -> std::result::Result<Self, Self::Error> {
        let mut params = HashMap::new();
        match value {
            File::FileID(fid) => {
                params.insert("fileid".to_string(), fid.0.to_string());
            }
            File::Path(p) => {
                params.insert("path".to_string(), p.to_string_lossy().parse()?);
            }
        };
        Ok(params)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::FileID;

    use super::*;

    #[test]
    fn test_params_with_fileid() {
        let input = File::FileID(FileID(1234));
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
    }

    #[test]
    fn test_params_with_path() {
        let input = File::from_str("/this/is/the/path").unwrap();
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("path"), Some(&"/this/is/the/path".to_string()));
    }
}

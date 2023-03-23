use std::collections::HashMap;

use anyhow::Result;

use crate::types::File;

use super::Params;

impl Params for File {
    fn add_to_params(&self, params: &mut HashMap<String, String>) -> Result<()> {
        match self {
            File::FileID(fid) => {
                params.insert("fileid".to_string(), fid.0.to_string());
            }
            File::RemotePath(p) => {
                params.insert("path".to_string(), p.path().to_string());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::FileID;

    use super::*;

    #[test]
    fn test_params_with_fileid() -> Result<()> {
        let input = File::FileID(FileID(1234));
        let mut params = HashMap::new();
        input.add_to_params(&mut params)?;
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
        Ok(())
    }

    #[test]
    fn test_params_with_path() -> Result<()> {
        let input = File::from_str("path:/this/is/the/path").unwrap();
        let mut params = HashMap::new();
        input.add_to_params(&mut params)?;
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("path"), Some(&"/this/is/the/path".to_string()));
        Ok(())
    }
}

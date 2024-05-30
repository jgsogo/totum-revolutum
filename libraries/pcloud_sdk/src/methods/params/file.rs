use std::collections::HashMap;

use anyhow::Result;

use crate::types::{File, FileID};

use super::{Params, ParamsType};

impl Params for FileID {
    fn add_to_params(&self, params: &mut ParamsType) -> Result<()> {
        params.insert("fileid".to_string(), self.inner().to_string());
        Ok(())
    }
}

impl Params for File {
    fn add_to_params(&self, params: &mut HashMap<String, String>) -> Result<()> {
        match self {
            File::FileID(fid) => fid.add_to_params(params),
            File::RemotePath(p) => p.add_to_params(params),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::FileID;

    use super::*;

    #[test]
    fn test_params_with_fileid() -> Result<()> {
        let input: File = FileID::new(1234).into();
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

use std::collections::HashMap;

use http_utils::AddToParams;

use crate::types::{File, FileID};

impl AddToParams for FileID {
    fn add_to_params(&self, params: &mut HashMap<String, String>) {
        params.insert("fileid".to_string(), self.inner().to_string());
    }
}

impl AddToParams for File {
    fn add_to_params(&self, params: &mut HashMap<String, String>) {
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
    fn test_params_with_fileid() {
        let input: File = FileID::new(1234).into();
        let mut params = HashMap::new();
        input.add_to_params(&mut params);
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
    }

    #[test]
    fn test_params_with_path() {
        let input = File::from_str("path:/this/is/the/path").unwrap();
        let mut params = HashMap::new();
        input.add_to_params(&mut params);
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("path"), Some(&"/this/is/the/path".to_string()));
    }
}

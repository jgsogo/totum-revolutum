use http_utils::AddToParams;
use std::collections::HashMap;

use crate::types::RemotePath;

impl AddToParams for RemotePath {
    fn add_to_params(&self, params: &mut HashMap<String, String>) {
        params.insert("path".to_string(), self.as_path().to_string());
    }
}

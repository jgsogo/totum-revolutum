use anyhow::Result;

use crate::types::RemotePath;

use super::{Params, ParamsType};

impl Params for RemotePath {
    fn add_to_params(&self, params: &mut ParamsType) -> Result<()> {
        params.insert("path".to_string(), self.path().to_string());
        Ok(())
    }
}

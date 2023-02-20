use std::collections::HashMap;

use anyhow::Result;

pub trait Params {
    fn add_to_params(&self, params: &mut HashMap<String, String>) -> Result<()>;
}

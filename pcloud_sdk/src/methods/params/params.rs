use std::collections::HashMap;

use anyhow::Result;

pub type ParamsType = HashMap<String, String>;

pub trait Params {
    fn add_to_params(&self, params: &mut ParamsType) -> Result<()>;

    fn into_params(self) -> Result<ParamsType>
    where
        Self: Sized,
    {
        let mut params = HashMap::new();
        self.add_to_params(&mut params)?;
        Ok(params)
    }
}

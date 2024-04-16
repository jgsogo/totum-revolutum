use crate::methods::params::{Params, ParamsType};

pub mod file_close;
pub mod file_open;
pub mod file_read;
pub mod file_write;

pub type FileDescriptor = u64;

impl Params for FileDescriptor {
    fn add_to_params(&self, params: &mut ParamsType) -> anyhow::Result<()> {
        params.insert("fd".to_string(), self.to_string());
        Ok(())
    }
}

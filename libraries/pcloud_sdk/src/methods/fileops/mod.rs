use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use utils::http::AddToParams;

pub mod file_close;
pub mod file_open;
pub mod file_read;
pub mod file_write;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct FileDescriptor(u64);

impl FileDescriptor {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

impl Display for FileDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileDescriptor({})", self.0)
    }
}

impl AddToParams for FileDescriptor {
    fn add_to_params(&self, params: &mut HashMap<String, String>) {
        params.insert("fd".to_string(), self.0.to_string());
    }
}

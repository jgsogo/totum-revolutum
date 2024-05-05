use anyhow::Result;
use filesystem::Filesystem;

pub struct PhotoDB {}

impl PhotoDB {
    pub fn instance<F: Filesystem>(root: F, initialize_if_not_exists: bool) -> Result<Self> {
        todo!();
    }
}

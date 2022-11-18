use crate::errors::SDKErrors;
use crate::storage::config;
use anyhow::{anyhow, Result};
use std::path::Path;
use tracing::info;

pub fn run(path: &Path, _config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

    std::thread::sleep(std::time::Duration::from_secs(5));
    info!("Done backup action on path '{}'", path.display());

    Err(anyhow!(SDKErrors::NotImplemented))
}

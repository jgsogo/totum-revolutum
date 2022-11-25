use std::path::Path;

use anyhow::Result;
use tracing::{error, info};

use crate::diff;
use crate::local;
use crate::remote;
use crate::storage::config;

pub async fn run(path: &Path, config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

    // TODO: Better to add all PATHS to the same walker than to instantiate a new one for each: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L610
    let (lhs, rhs, mut report) =
        diff::two_ways::TwoWaysDiff::<local::LocalMetadata, remote::RemoteMetadata>::new();

    let res = tokio::try_join!(
        local::walk_local_directory(path, 6, lhs),
        remote::walk_remote_directory(config, 6, rhs),
        report.recv(),
    );

    match res {
        Ok((_, _, _)) => report.report().await?,
        Err(err) => {
            error!("Found error while computing the diff: {}", err);
            return Err(err);
        }
    }

    todo!("dasffda")
}

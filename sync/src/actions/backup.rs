use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use tracing::{debug, error, info};

use crate::diff;
use crate::diff::basepoint::{FileMetadata, SnapshotStatus};
use crate::diff::two_ways::FileDiff;
use crate::local;
use crate::remote;
use crate::storage::config;

fn handle_filediff<LHSMetadata: FileMetadata, RHSMetadata: FileMetadata>(
    file_diff: FileDiff<LHSMetadata, RHSMetadata>,
) {
    let status = match &file_diff {
        FileDiff {
            lhs_metadata: Some(lhs_metadata),
            rhs_metadata: Some(rhs_metadata),
        } => {
            if lhs_metadata.eq(rhs_metadata) {
                SnapshotStatus::Idle
            } else {
                SnapshotStatus::Modified
            }
        }
        FileDiff {
            lhs_metadata: Some(_),
            ..
        } => SnapshotStatus::New,
        FileDiff {
            rhs_metadata: Some(_),
            ..
        } => SnapshotStatus::ToBeDeleted,
        _ => panic!("Not expected"),
    };

    debug!("{:?} | {}", status, file_diff.id());
    match status {
        SnapshotStatus::Idle => (),
        e => println!("{:?} | {}", e, file_diff.id()),
    }
}

async fn work_on_results<
    LHSMetadata: FileMetadata + 'static,
    RHSMetadata: FileMetadata + 'static,
>(
    rx: flume::Receiver<FileDiff<LHSMetadata, RHSMetadata>>,
) -> Result<()> {
    tokio::spawn(async move {
        info!("Start backup receiving loop");
        let start = Instant::now();
        while let Ok(v) = rx.recv_async().await {
            handle_filediff(v);
        }
        info!("Finished backup receiving loop in {:?}", start.elapsed());
    })
    .await?;
    Ok(())
}

pub async fn run(path: &Path, config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

    // TODO: Better to add all PATHS to the same walker than to instantiate a new one for each: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L610
    let (lhs, rhs, differ) =
        diff::two_ways::run::<local::LocalMetadata, remote::RemoteMetadata>().await;

    if let Err(e) = tokio::try_join!(
        work_on_results(differ),
        local::walk_local_directory(path, 6, lhs),
        remote::walk_remote_directory(config, 6, rhs),
        // work_on_results(differ),
    ) {
        error!("Error on workers loop: {e}");
    }

    todo!("dasffda")
}

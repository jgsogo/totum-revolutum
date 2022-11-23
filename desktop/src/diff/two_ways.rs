use std::collections::{hash_map::Entry, HashMap};

use super::basepoint::{BasePointDiffImpl, FileMetadata, SnapshotStatus, MAX_BUFFER};

use anyhow::Result;
use tracing::{debug, info};

struct FileDiff<LHSMetadata: FileMetadata, RHSMetadata: FileMetadata> {
    pub lhs_metadata: Option<LHSMetadata>,
    pub rhs_metadata: Option<RHSMetadata>,
}

impl<LHSMetadata, RHSMetadata> FileDiff<LHSMetadata, RHSMetadata>
where
    LHSMetadata: FileMetadata,
    RHSMetadata: FileMetadata,
{
    pub fn new(
        lhs: Option<LHSMetadata>,
        rhs: Option<RHSMetadata>,
    ) -> FileDiff<LHSMetadata, RHSMetadata> {
        FileDiff::<LHSMetadata, RHSMetadata> {
            lhs_metadata: lhs,
            rhs_metadata: rhs,
        }
    }
}

pub struct TwoWaysDiff<LHSMetadata: FileMetadata, RHSMetadata: FileMetadata> {
    lhs_rx: flume::Receiver<LHSMetadata>,
    rhs_rx: flume::Receiver<RHSMetadata>,

    files: HashMap<String, FileDiff<LHSMetadata, RHSMetadata>>,
}

impl<LHSMetadata, RHSMetadata> TwoWaysDiff<LHSMetadata, RHSMetadata>
where
    LHSMetadata: FileMetadata,
    RHSMetadata: FileMetadata,
{
    pub fn new() -> (
        BasePointDiffImpl<LHSMetadata>,
        BasePointDiffImpl<RHSMetadata>,
        TwoWaysDiff<LHSMetadata, RHSMetadata>,
    ) {
        let (lhs_tx, lhs_rx) = flume::bounded::<LHSMetadata>(MAX_BUFFER);
        let (rhs_tx, rhs_rx) = flume::bounded::<RHSMetadata>(MAX_BUFFER);
        let lhs = BasePointDiffImpl::<LHSMetadata>::new(lhs_tx);
        let rhs = BasePointDiffImpl::<RHSMetadata>::new(rhs_tx);

        let report = TwoWaysDiff::<LHSMetadata, RHSMetadata> {
            lhs_rx,
            rhs_rx,
            files: HashMap::default(),
        };
        (lhs, rhs, report)
    }

    pub async fn recv(&mut self) -> Result<()> {
        info!("Start receiving loop");
        loop {
            tokio::select! {
                Ok(lhs) = self.lhs_rx.recv_async() => {
                    debug!("LHS received {:?}", lhs);
                    match self.files.entry(lhs.id().to_string()) {
                        Entry::Occupied(o) => {
                            o.into_mut().lhs_metadata = Some(lhs);
                        },
                        Entry::Vacant(v) => {
                            v.insert(FileDiff::<LHSMetadata, RHSMetadata>::new(Some(lhs), None));
                        },
                    };
                },
                Ok(rhs) = self.rhs_rx.recv_async() => {
                    debug!("RHS received {:?}", rhs);
                    match self.files.entry(rhs.id().to_string()) {
                        Entry::Occupied(o) => {
                            o.into_mut().rhs_metadata = Some(rhs);
                        },
                        Entry::Vacant(v) => {
                            v.insert(FileDiff::<LHSMetadata, RHSMetadata>::new(None, Some(rhs)));
                        },
                    };
                },
                else => {
                    info!("Finished receiving loop");
                    break
                },
            }
        }
        Ok(())
    }

    pub async fn report(&mut self) -> Result<()> {
        self.recv().await?;

        println!("We have {} entries", self.files.len());
        for (key, value) in self.files.iter() {
            let status = match value {
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
                } => SnapshotStatus::ToBeDeleted,
                FileDiff {
                    rhs_metadata: Some(_),
                    ..
                } => SnapshotStatus::New,
                _ => panic!("Not expected"),
            };

            println!(
                "{:?} | {} | {} / {}",
                status,
                key,
                value
                    .lhs_metadata
                    .as_ref()
                    .map_or("-".into(), |v| v.size().to_string()),
                value
                    .rhs_metadata
                    .as_ref()
                    .map_or("-".into(), |v| v.size().to_string())
            );
        }
        Ok(())
    }
}

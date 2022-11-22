use super::basepoint::{BasePointDiffImpl, FileMetadata, MAX_BUFFER};

use tracing::info;

pub struct TwoWaysDiff<LHSMetadata: FileMetadata, RHSMetadata: FileMetadata> {
    lhs_rx: flume::Receiver<LHSMetadata>,
    rhs_rx: flume::Receiver<RHSMetadata>,
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

        let report = TwoWaysDiff::<LHSMetadata, RHSMetadata> { lhs_rx, rhs_rx };
        (lhs, rhs, report)
    }

    pub async fn recv(&mut self) {
        info!("Start receiving loop");
        loop {
            tokio::select! {
                Ok(lhs) = self.lhs_rx.recv_async() => {
                    info!("LHS received {:?}", lhs);
                },
                Ok(rhs) = self.rhs_rx.recv_async() => {
                    info!("RHS received {:?}", rhs);
                },
                else => {
                    info!("Finished receiving loop");
                    break
                },
            }
        }
    }
}

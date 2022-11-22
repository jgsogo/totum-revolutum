use super::basepoint::{BasePointDiffImpl, FileMetadata};
use tokio::sync::mpsc;
use tracing::info;

pub struct TwoWaysDiff<LHSMetadata: FileMetadata, RHSMetadata: FileMetadata> {
    lhs_rx: mpsc::UnboundedReceiver<LHSMetadata>,
    rhs_rx: mpsc::UnboundedReceiver<RHSMetadata>,
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
        let (lhs_tx, lhs_rx) = mpsc::unbounded_channel::<LHSMetadata>();
        let (rhs_tx, rhs_rx) = mpsc::unbounded_channel::<RHSMetadata>();
        let lhs = BasePointDiffImpl::<LHSMetadata>::new(lhs_tx);
        let rhs = BasePointDiffImpl::<RHSMetadata>::new(rhs_tx);

        let report = TwoWaysDiff::<LHSMetadata, RHSMetadata> { lhs_rx, rhs_rx };
        (lhs, rhs, report)
    }

    pub async fn recv(&mut self) {
        info!("Start receiving loop");
        loop {
            tokio::select! {
                Some(lhs) = self.lhs_rx.recv() => {
                    info!("LHS received {:?}", lhs);
                },
                Some(rhs) = self.rhs_rx.recv() => {
                    info!("RHS received {:?}", rhs);
                },
                else => break,
            }
        }
    }
}

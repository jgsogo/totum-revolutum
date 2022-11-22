use super::basepoint::{BasePointDiffImpl, FileMetadata};
use tokio::sync::mpsc;

pub struct TwoWaysDiff<LHSMetadata: FileMetadata, RHSMetadata: FileMetadata> {
    lhs: BasePointDiffImpl<LHSMetadata>,
    rhs: BasePointDiffImpl<RHSMetadata>,
}

impl<LHSMetadata, RHSMetadata> TwoWaysDiff<LHSMetadata, RHSMetadata>
where
    LHSMetadata: FileMetadata + std::marker::Send,
    RHSMetadata: FileMetadata + std::marker::Send,
{
    pub fn new() -> TwoWaysDiff<LHSMetadata, RHSMetadata> {
        let (lhs_tx, lhs_rx) = mpsc::unbounded_channel::<LHSMetadata>();
        let (rhs_tx, rhs_rx) = mpsc::unbounded_channel::<RHSMetadata>();
        TwoWaysDiff::<LHSMetadata, RHSMetadata> {
            lhs: BasePointDiffImpl::<LHSMetadata>::new(lhs_tx),
            rhs: BasePointDiffImpl::<RHSMetadata>::new(rhs_tx),
        }
    }

    pub fn lhs(&self) -> &BasePointDiffImpl<LHSMetadata> {
        &self.lhs
    }
    pub fn rhs(&self) -> &BasePointDiffImpl<RHSMetadata> {
        &self.rhs
    }
}

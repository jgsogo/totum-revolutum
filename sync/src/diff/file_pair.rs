use crate::diff::FileMetadata;

/// Contains a pair of [`FileMetadata`]
///
/// It is used tipically on a two way diff algorithm to return the results from the diff, where
/// the tuple contains one file (`lhs`) from the LHS filesystem, and the other file (`rhs`) from
/// the RHS filesystem.
pub struct FilePair<LHS: FileMetadata, RHS: FileMetadata> {
    pub lhs: Option<LHS>,
    pub rhs: Option<RHS>,
}

impl<LHS: FileMetadata, RHS: FileMetadata> FilePair<LHS, RHS> {
    fn new(lhs: Option<LHS>, rhs: Option<RHS>) -> FilePair<LHS, RHS> {
        FilePair::<LHS, RHS> { lhs, rhs }
    }

    pub fn new_from_lhs(lhs: LHS) -> FilePair<LHS, RHS> {
        Self::new(Some(lhs), None)
    }

    pub fn new_from_rhs(rhs: RHS) -> FilePair<LHS, RHS> {
        Self::new(None, Some(rhs))
    }

    pub fn id(&self) -> &str {
        self.lhs
            .as_ref()
            .map_or_else(|| self.rhs.as_ref().unwrap().id(), |v| v.id())
    }
}

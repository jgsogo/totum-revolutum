use crate::FileMetadata;

/// Contains a pair of [`FileMetadata`]
///
/// It is used typically on a two way diff algorithm to return the results from the diff, where
/// the tuple contains one file (`lhs`) from the LHS [`crate::Filesystem`], and the other file
/// (`rhs`) from the RHS [`crate::Filesystem`].
pub struct FilePair {
    pub lhs: Option<Box<dyn FileMetadata>>,
    pub rhs: Option<Box<dyn FileMetadata>>,
}

impl FilePair {
    fn new(lhs: Option<Box<dyn FileMetadata>>, rhs: Option<Box<dyn FileMetadata>>) -> FilePair {
        FilePair { lhs, rhs }
    }

    pub fn new_from_lhs(lhs: Box<dyn FileMetadata>) -> FilePair {
        Self::new(Some(lhs), None)
    }

    pub fn new_from_rhs(rhs: Box<dyn FileMetadata>) -> FilePair {
        Self::new(None, Some(rhs))
    }

    pub fn id(&self) -> &str {
        self.lhs
            .as_ref()
            .map_or_else(|| self.rhs.as_ref().unwrap().path(), |v| v.path())
            .as_str()
    }
}

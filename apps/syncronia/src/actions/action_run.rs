use anyhow::anyhow;
use async_trait::async_trait;

use filesystem::diff::FilePair;
use filesystem::FileMetadata;

#[async_trait]
pub trait ActionRun: Sync {
    async fn run(&self, file_pair: FilePair) -> anyhow::Result<()> {
        match file_pair {
            // Both files exist
            FilePair {
                lhs: Some(lhs),
                rhs: Some(rhs),
            } => {
                if lhs.eq(rhs.as_ref())? {
                    self.run_with_both_eq(lhs.as_ref(), rhs.as_ref()).await
                } else {
                    self.run_with_both(lhs.as_ref(), rhs.as_ref()).await
                }
            }
            // Only LHS exist
            FilePair { lhs: Some(lhs), .. } => self.run_with_lhs(lhs.as_ref()).await,
            // Only RHS exist
            FilePair { rhs: Some(rhs), .. } => self.run_with_rhs(rhs.as_ref()).await,
            // None exist
            _ => Err(anyhow!("None sides of the file exist, unexpected error!")),
        }
    }

    async fn run_with_both_eq(&self, _lhs: &dyn FileMetadata, _rhs: &dyn FileMetadata) -> anyhow::Result<()> {
        Ok(())
    }

    async fn run_with_both(&self, _lhs: &dyn FileMetadata, _rhs: &dyn FileMetadata) -> anyhow::Result<()> {
        Ok(())
    }

    async fn run_with_lhs(&self, _lhs: &dyn FileMetadata) -> anyhow::Result<()> {
        Ok(())
    }

    async fn run_with_rhs(&self, _rhs: &dyn FileMetadata) -> anyhow::Result<()> {
        Ok(())
    }

    fn stats(&self) {
        todo!("Print stats for actions performed")
    }
}

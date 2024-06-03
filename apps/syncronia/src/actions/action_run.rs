use anyhow::anyhow;
use async_trait::async_trait;

use filesystem::diff::FilePair;
use filesystem::FileMetadata;

#[async_trait]
pub trait ActionRun<FsLhsMetadata: FileMetadata + 'static, FsRhsMetadata: FileMetadata + 'static>: Sync {
    async fn run(&self, file_pair: FilePair<FsLhsMetadata, FsRhsMetadata>) -> anyhow::Result<()> {
        match file_pair {
            // Both files exist
            FilePair {
                lhs: Some(lhs),
                rhs: Some(rhs),
            } => {
                if lhs.eq(&rhs)? {
                    self.run_with_both_eq(&lhs, &rhs).await
                } else {
                    self.run_with_both(&lhs, &rhs).await
                }
            }
            // Only LHS exist
            FilePair { lhs: Some(lhs), .. } => self.run_with_lhs(&lhs).await,
            // Only RHS exist
            FilePair { rhs: Some(rhs), .. } => self.run_with_rhs(&rhs).await,
            // None exist
            _ => Err(anyhow!("None sides of the file exist, unexpected error!")),
        }
    }

    async fn run_with_both_eq(&self, _lhs: &FsLhsMetadata, _rhs: &FsRhsMetadata) -> anyhow::Result<()> {
        Ok(())
    }

    async fn run_with_both(&self, _lhs: &FsLhsMetadata, _rhs: &FsRhsMetadata) -> anyhow::Result<()> {
        Ok(())
    }

    async fn run_with_lhs(&self, _lhs: &FsLhsMetadata) -> anyhow::Result<()> {
        Ok(())
    }

    async fn run_with_rhs(&self, _rhs: &FsRhsMetadata) -> anyhow::Result<()> {
        Ok(())
    }

    fn stats(&self) {
        todo!("Print stats for actions performed")
    }
}

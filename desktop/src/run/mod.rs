use anyhow::Result;
use std::path::{Path, PathBuf};
mod global;
mod project;

#[derive(Debug)]
pub enum RunCommand {
    Global,
    Directory(PathBuf),
}

pub async fn handle(home: &Path, command: RunCommand) -> Result<()> {
    match command {
        RunCommand::Global => global::handle(home).await,
        RunCommand::Directory(path) => project::handle(home, &path),
    }
}

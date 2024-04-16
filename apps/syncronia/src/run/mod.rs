use camino::{Utf8Path, Utf8PathBuf};

use anyhow::Result;

mod global;
mod project;

#[derive(Debug)]
pub enum RunCommand {
    Global,
    Directory(Utf8PathBuf),
}

pub async fn handle(home: &Utf8Path, command: RunCommand) -> Result<()> {
    match command {
        RunCommand::Global => global::handle(home).await,
        RunCommand::Directory(path) => project::handle(home, &path).await,
    }
}

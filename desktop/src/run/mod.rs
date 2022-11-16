use std::path::{Path, PathBuf};

mod global;
mod project;

#[derive(Debug)]
pub enum RunCommand {
    Global,
    Directory(PathBuf),
}

pub fn handle(home: &Path, command: RunCommand) {
    match command {
        RunCommand::Global => global::handle(home),
        RunCommand::Directory(path) => project::handle(home, &path),
    }
}

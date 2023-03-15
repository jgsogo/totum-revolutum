use anyhow::Result;
use camino::Utf8PathBuf;

pub use default_output::DefaultOutput;
use pcloud_sdk::methods::folder::listfolder::ListFolder;
use pcloud_sdk::methods::general::userinfo::UserInfo;
use pcloud_sdk::progress_bar::{ProgressBar, ProgressBarBuilder};
pub use porcelain::Porcelain;

use crate::CliParams;

mod default_output;
mod porcelain;

pub trait Print: ProgressBarBuilder {
    fn println(&self, text: &str) -> Result<()>;
    fn eprintln(&self, text: &str) -> Result<()>;

    fn path(&self, path: Utf8PathBuf) -> Result<()>;
    fn list_folder(&self, list_folder: &ListFolder) -> Result<()>;
    fn user_info(&self, user_info: &UserInfo) -> Result<()>;
}

pub enum PrintVariant {
    Default(DefaultOutput),
    Porcelain(Porcelain),
}

#[derive(clap::ValueEnum, Clone)]
pub enum OutputArg {
    /// Print as much information as possible
    Default,

    /// Gives the output in an easy to parse format. Usually just one data or one data per line.
    Porcelain,
}

impl PrintVariant {
    pub fn new(value: OutputArg, cli_params: &CliParams) -> Self {
        match value {
            OutputArg::Default => PrintVariant::Default(DefaultOutput::new(cli_params)),
            OutputArg::Porcelain => PrintVariant::Porcelain(Porcelain {}),
        }
    }
}

impl Print for PrintVariant {
    // TODO: There is a lot of boilerplate here just to forward a funciton call -- macro?
    fn println(&self, value: &str) -> Result<()> {
        match &self {
            PrintVariant::Default(inner) => inner.println(value),
            PrintVariant::Porcelain(inner) => inner.println(value),
        }
    }

    fn eprintln(&self, value: &str) -> Result<()> {
        match &self {
            PrintVariant::Default(inner) => inner.eprintln(value),
            PrintVariant::Porcelain(inner) => inner.eprintln(value),
        }
    }

    fn path(&self, path: Utf8PathBuf) -> Result<()> {
        match &self {
            PrintVariant::Default(inner) => inner.path(path),
            PrintVariant::Porcelain(inner) => inner.path(path),
        }
    }

    fn list_folder(&self, value: &ListFolder) -> Result<()> {
        match &self {
            PrintVariant::Default(inner) => inner.list_folder(value),
            PrintVariant::Porcelain(inner) => inner.list_folder(value),
        }
    }

    fn user_info(&self, value: &UserInfo) -> Result<()> {
        match &self {
            PrintVariant::Default(inner) => inner.user_info(value),
            PrintVariant::Porcelain(inner) => inner.user_info(value),
        }
    }
}

impl ProgressBarBuilder for PrintVariant {
    fn build(&self, total_size: u64) -> Box<dyn ProgressBar> {
        match &self {
            PrintVariant::Default(inner) => inner.build(total_size),
            PrintVariant::Porcelain(inner) => inner.build(total_size),
        }
    }
}

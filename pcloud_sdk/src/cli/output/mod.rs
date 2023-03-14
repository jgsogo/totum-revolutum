use anyhow::Result;

pub use default::Default;
use pcloud_sdk::methods::folder::listfolder::ListFolder;
use pcloud_sdk::methods::general::userinfo::UserInfo;
use pcloud_sdk::progress_bar::{ProgressBar, ProgressBarBuilder};
pub use porcelain::Porcelain;

mod default;
mod porcelain;

pub trait Print: ProgressBarBuilder {
    fn print(&self, text: &str);
    fn println(&self, text: &str);
    fn eprint(&self, text: &str);
    fn eprintln(&self, text: &str);

    fn list_folder(&self, list_folder: &ListFolder) -> Result<()>;
    fn user_info(&self, user_info: &UserInfo) -> Result<()>;
}

pub enum PrintVariant {
    Default(Default),
    Porcelain(Porcelain),
}

#[derive(clap::ValueEnum, Clone)]
pub enum OutputArg {
    /// Print as much information as possible
    Default,

    /// Gives the output in an easy to parse format. Usually just one data or one data per line.
    Porcelain,
}

impl From<OutputArg> for PrintVariant {
    fn from(value: OutputArg) -> Self {
        match value {
            OutputArg::Default => PrintVariant::Default(Default {}),
            OutputArg::Porcelain => PrintVariant::Porcelain(Porcelain {}),
        }
    }
}

impl Print for PrintVariant {
    // TODO: There is a lot of boilerplate here just to forward a funciton call -- macro?
    fn print(&self, value: &str) {
        match &self {
            PrintVariant::Default(inner) => inner.print(value),
            PrintVariant::Porcelain(inner) => inner.print(value),
        }
    }

    fn println(&self, value: &str) {
        match &self {
            PrintVariant::Default(inner) => inner.println(value),
            PrintVariant::Porcelain(inner) => inner.println(value),
        }
    }

    fn eprint(&self, value: &str) {
        match &self {
            PrintVariant::Default(inner) => inner.eprint(value),
            PrintVariant::Porcelain(inner) => inner.eprint(value),
        }
    }

    fn eprintln(&self, value: &str) {
        match &self {
            PrintVariant::Default(inner) => inner.eprintln(value),
            PrintVariant::Porcelain(inner) => inner.eprintln(value),
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

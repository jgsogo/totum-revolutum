use anyhow::Result;

pub use debug::Debug;
use pcloud_sdk::methods::folder::listfolder::ListFolder;
use pcloud_sdk::methods::general::userinfo::UserInfo;
pub use porcelain::Porcelain;

mod debug;
mod porcelain;

pub trait Print {
    fn list_folder(&self, list_folder: &ListFolder) -> Result<()>;
    fn user_info(&self, user_info: &UserInfo) -> Result<()>;
}

pub enum PrintVariant {
    Debug(Debug),
    Porcelain(Porcelain),
}

#[derive(clap::ValueEnum, Clone)]
pub enum OutputArg {
    /// Print as much information as possible
    Debug,
    /// Gives the output in an easy to parse format. Usually just one data or one data per line.
    Porcelain,
}

impl From<OutputArg> for PrintVariant {
    fn from(value: OutputArg) -> Self {
        match value {
            OutputArg::Debug => PrintVariant::Debug(Debug {}),
            OutputArg::Porcelain => PrintVariant::Porcelain(Porcelain {}),
        }
    }
}

impl Print for PrintVariant {
    // TODO: There is a lot of boilerplate here just to forward a funciton call -- macro?
    fn list_folder(&self, value: &ListFolder) -> Result<()> {
        match &self {
            PrintVariant::Debug(inner) => inner.list_folder(value),
            PrintVariant::Porcelain(inner) => inner.list_folder(value),
        }
    }

    fn user_info(&self, value: &UserInfo) -> Result<()> {
        match &self {
            PrintVariant::Debug(inner) => inner.user_info(value),
            PrintVariant::Porcelain(inner) => inner.user_info(value),
        }
    }
}

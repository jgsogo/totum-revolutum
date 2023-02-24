use anyhow::{bail, Result};

use pcloud_sdk::methods::folder::ListFolder;
use pcloud_sdk::methods::general::UserInfo;
use pcloud_sdk::structures::Metadata;

use crate::output::Print;

/// Output prepared to be piped to other commands. Lines will be consumed
/// by subsequent commands one by one (everything after `//` is ignored)
pub struct Porcelain;

impl Porcelain {
    fn format_line(data: &str, comment: &str) {
        println!("{}\t// {}", data, comment);
    }
}

impl Print for Porcelain {
    fn list_folder(&self, list_folder: &ListFolder) -> Result<()> {
        if let Some(contents) = &list_folder.metadata.contents {
            for it in contents.iter() {
                match it {
                    Metadata {
                        common,
                        fileid: Some(fd),
                        ..
                    } => Porcelain::format_line(&fd.to_string(), common.name.as_ref().unwrap_or(&"".to_string())),
                    Metadata {
                        common,
                        folderid: Some(fd),
                        ..
                    } => Porcelain::format_line(&fd.to_string(), common.name.as_ref().unwrap_or(&"".to_string())),
                    _ => bail!("Not a folder, neither a file"),
                }
            }
        }
        Ok(())
    }

    fn user_info(&self, user_info: &UserInfo) -> Result<()> {
        Porcelain::format_line(
            &user_info.email,
            &format!(
                "id:{}, premium:{}, quota:{}/{} Gb",
                user_info.userid,
                user_info.premium,
                user_info.usedquota_in_gigas(),
                user_info.quota_in_gigas()
            ),
        );
        Ok(())
    }
}

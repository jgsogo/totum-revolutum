use anyhow::{bail, Result};

use pcloud_sdk::methods::folder::ListFolder;
use pcloud_sdk::methods::general::UserInfo;
use pcloud_sdk::structures::Metadata;

use crate::output::Print;

const COMMENT_SEPARATOR: &str = "//";

/// Output prepared to be piped to other commands. Lines will be consumed
/// by subsequent commands one by one (everything after `//` is ignored)
pub struct Porcelain;

impl Porcelain {
    fn format_line(data: &str, comment: &str) -> String {
        if data.is_empty() {
            format!("{} {}", COMMENT_SEPARATOR, comment)
        } else {
            format!("{}\t{} {}", data, COMMENT_SEPARATOR, comment)
        }
    }

    pub fn parse_line(line: &str) -> String {
        let parts: Vec<&str> = line.splitn(2, COMMENT_SEPARATOR).collect();
        parts.into_iter().nth(0).unwrap().trim().to_string()
    }
}

impl Print for Porcelain {
    // Forward messages to stderr, as porcelain output reduces noise as much as possible
    fn print(&self, text: &str) {
        eprint!("{text}");
    }

    // Forward messages to stderr, as porcelain output reduces noise as much as possible
    fn println(&self, text: &str) {
        eprintln!("{text}");
    }

    fn eprint(&self, text: &str) {
        eprint!("{text}");
    }

    fn eprintln(&self, text: &str) {
        eprintln!("{text}");
    }

    fn list_folder(&self, list_folder: &ListFolder) -> Result<()> {
        if let Some(contents) = &list_folder.metadata.contents {
            println!(
                "{}",
                Porcelain::format_line("", &list_folder.metadata.folderid.as_ref().unwrap().to_string())
            );

            for it in contents.iter() {
                let line = match it {
                    Metadata {
                        common,
                        fileid: Some(fd),
                        ..
                    } => Porcelain::format_line(&fd.to_string(), common.name.as_ref().unwrap_or(&"".to_string())),
                    Metadata {
                        common,
                        folderid: Some(fd),
                        ..
                    } => Porcelain::format_line(
                        &fd.to_string(),
                        &format!("{}/", common.name.as_ref().unwrap_or(&"".to_string())),
                    ),
                    _ => bail!("Not a folder, neither a file"),
                };
                println!("{line}");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let data = "data";
        let comment = "comment";
        let line = Porcelain::format_line(data, comment);

        let parsed_data = Porcelain::parse_line(&line);
        assert_eq!(data, parsed_data);
    }

    #[test]
    fn test_parse_line_with_comments() {
        let data = "data";
        let comment = &format!("comment {} more comment", COMMENT_SEPARATOR);
        let line = Porcelain::format_line(data, comment);

        let parsed_data = Porcelain::parse_line(&line);
        assert_eq!(data, parsed_data);
    }

    #[test]
    fn test_parse_just_comment() {
        let data = "";
        let comment = &format!("comment {} more comment", COMMENT_SEPARATOR);
        let line = Porcelain::format_line(data, comment);

        let parsed_data = Porcelain::parse_line(&line);
        assert_eq!(data, parsed_data);
    }
}

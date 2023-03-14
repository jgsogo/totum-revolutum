use anyhow::Result;

use pcloud_sdk::methods::folder::ListFolder;
use pcloud_sdk::methods::general::UserInfo;

use crate::output::Print;

pub struct Default;

impl Print for Default {
    fn print(&self, text: &str) {
        print!("{text}");
    }

    fn println(&self, text: &str) {
        println!("{text}");
    }

    fn eprint(&self, text: &str) {
        eprint!("{text}");
    }

    fn eprintln(&self, text: &str) {
        eprintln!("{text}");
    }

    fn list_folder(&self, list_folder: &ListFolder) -> Result<()> {
        println!("{:?}", list_folder);
        Ok(())
    }

    fn user_info(&self, user_info: &UserInfo) -> Result<()> {
        println!("{:?}", user_info);
        Ok(())
    }
}

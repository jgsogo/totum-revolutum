use std::{env, path::Path, path::PathBuf};

use anyhow::Result;
use clap::Args;

const PCLOUD_HOME_SUBDIR: &str = ".pcloud";
const PCLOUD_HOME_DIR_ENVVAR: &str = "PCLOUD_HOME_DIR";

/// Print home folder. Change it using env variable `PCLOUD_HOME_DIR`
#[derive(Args, Debug)]
pub struct HomeParams {}

pub fn pcloud_home() -> PathBuf {
    match env::var(PCLOUD_HOME_DIR_ENVVAR) {
        Ok(p) => {
            let p = PathBuf::from(&p);
            if p.is_relative() {
                eprintln!(
                    "{} ('{}') needs to be an absolute path",
                    PCLOUD_HOME_DIR_ENVVAR,
                    p.display()
                );
                std::process::exit(1);
            }
            p
        }
        Err(_) => match home::home_dir() {
            Some(h) => h.join(PCLOUD_HOME_SUBDIR),
            None => {
                eprintln!("Provide home directory for pCloud");
                std::process::exit(1);
            }
        },
    }
}

pub fn handle(home_dir: &Path, _params: &HomeParams) -> Result<()> {
    println!("{}", home_dir.display());
    Ok(())
}

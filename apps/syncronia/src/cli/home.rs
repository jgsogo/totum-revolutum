use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use clap::Args;
use std::env;

const PCLOUD_HOME_SUBDIR: &str = ".pcloud";
const PCLOUD_HOME_DIR_ENVVAR: &str = "PCLOUD_HOME_DIR";

/// Print home folder. Change it using env variable `PCLOUD_HOME_DIR`
#[derive(Args, Debug)]
pub struct HomeParams {}

pub fn pcloud_home() -> Utf8PathBuf {
    match env::var(PCLOUD_HOME_DIR_ENVVAR) {
        Ok(p) => {
            let p = Utf8PathBuf::from(&p);
            if p.is_relative() {
                eprintln!("{} ('{p}') needs to be an absolute path", PCLOUD_HOME_DIR_ENVVAR,);
                std::process::exit(1);
            }
            p
        }
        Err(_) => match home::home_dir() {
            Some(h) => Utf8PathBuf::from_path_buf(h).unwrap().join(PCLOUD_HOME_SUBDIR),
            None => {
                eprintln!("Provide home directory for pCloud");
                std::process::exit(1);
            }
        },
    }
}

pub fn handle(home_dir: &Utf8Path, _params: &HomeParams) -> Result<()> {
    println!("{home_dir}");
    Ok(())
}

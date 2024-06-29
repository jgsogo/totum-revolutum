use camino::Utf8PathBuf;
use std::env;
use std::str::FromStr;

pub mod client;
pub mod filesystem;
pub mod server;

pub fn manifest_dir() -> Utf8PathBuf {
    let manifest_dir = match env::var("BAZEL_TEST") {
        Ok(_) => {
            let current_path = env::current_dir().unwrap();
            current_path.join("libraries/pcloud_sdk").to_str().unwrap().to_string()
        }
        Err(_) => env!("CARGO_MANIFEST_DIR").to_string(),
    };

    Utf8PathBuf::from_str(&manifest_dir).unwrap()
}

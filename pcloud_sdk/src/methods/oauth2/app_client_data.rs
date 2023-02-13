use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppClientData {
    pub client_id: String,
    pub client_secret: String,
    pub force_reapprove: bool,
}

impl AppClientData {
    pub fn new(client_id: &str, client_secret: &str) -> AppClientData {
        AppClientData {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            force_reapprove: false,
        }
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<AppClientData> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let u = serde_json::from_reader(reader)?;
        Ok(u)
    }
}

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Stores the information required to perform an authorization request. It can be
/// created manually using [`AppClientData::new`] or retrieved from a JSON file
/// with [`AppClientData::read_from_file`] method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppClientData {
    client_id: String,
    client_secret: String,
    force_reapprove: bool,
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

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    pub fn force_reapprove(&self) -> bool {
        self.force_reapprove
    }
}

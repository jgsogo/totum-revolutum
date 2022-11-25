use serde::{Deserialize, Serialize};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct VersionedData<T> {
    version: String,

    #[serde(flatten)]
    pub data: T,
}

impl<T> VersionedData<T> {
    fn default(default: T) -> Self {
        Self {
            version: VERSION.to_string(),
            data: default,
        }
    }
}

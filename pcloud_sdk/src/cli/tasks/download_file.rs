use pcloud_sdk::types::File;

use crate::tasks::Task;

struct DownloadFile {
    file: File,
}

impl DownloadFile {
    pub fn new(file: File) -> Self {
        Self { file }
    }
}

impl Task for DownloadFile {}

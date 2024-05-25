use crate::client::PCloudClient;
use crate::create_side_task;
use crate::types::{FileID, FolderID};
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{tempdir, TempDir};
use tokio::sync::oneshot::{Receiver, Sender};
use tracing::error;

/// Keeps a local temporal copy of a remote file. The remote file is fetched (or created) as
/// soon as this object is instantiated, then you can use the local file as usual. When this
/// object is dropped, the local content is sent to the remote and the local file is removed.
pub struct ProxiedFile<PCloud: PCloudClient + Send + 'static> {
    /// The `PCloudClient` used to connect to the remote
    pcloud: Option<PCloud>,

    /// Remote `FileID`
    file_id: FileID,

    /// The temporal directory where the local file is located. It will be removed after dropped.
    tmpdir: TempDir,

    /// The `File` object. It's created on demand when the file is opened.
    local_file: Option<File>,

    /// Signal to trigger inside `ProxiedFile::drop` so the local file is uploaded to the remote
    drop_trigger: Option<Sender<(PCloud, FileID, PathBuf)>>,

    /// Signal received after the local file is uploaded
    drop_completion: Option<Receiver<Result<Result<()>>>>,
}

async fn upload<PCloud: PCloudClient + Send>(_args: (PCloud, FileID, PathBuf)) -> Result<()> {
    Ok(())
}

impl<PCloud: PCloudClient + Send + 'static> ProxiedFile<PCloud> {
    /// Creates a local temporal file with the contents of the remote one (it will also create the
    /// remote one if it doesn't exist yet). Returns a tuple with the `ProxiedFile` object and
    /// a boolean indicating if the remote file was created or not.
    pub fn new(pcloud: PCloud, _folder_id: FolderID, _filename: &str) -> Result<(Self, bool)> {
        // Get or create the remote FileID
        let (file_id, created): (FileID, bool) = {
            // TODO: implement here
            (FileID(0), true)
        };

        // Create a side task, so we can execute the async upload from the Drop method
        let (trigger, completion) = create_side_task(upload);

        // Return to the user
        Ok((
            Self {
                pcloud: Some(pcloud),
                file_id,
                tmpdir: tempdir()?,
                local_file: None,
                drop_trigger: Some(trigger),
                drop_completion: Some(completion),
            },
            created,
        ))
    }

    /// Returns the full path to the local filename
    pub fn local_filepath(&self) -> PathBuf {
        self.tmpdir.path().join("proxied_file.tmp")
    }

    /// Returns a reference to a `std::fs::File` object. The value is kept inside the `ProxiedFile`
    /// so it not possible to drop it while the file is opened.
    pub fn open(&mut self) -> Result<&File> {
        if self.local_file.is_none() {
            self.local_file = Some(File::open(self.local_filepath())?);
        }
        Ok(self.local_file.as_ref().unwrap())
    }
}

impl<PCloud: PCloudClient + Send + 'static> Drop for ProxiedFile<PCloud> {
    fn drop(&mut self) {
        match self.local_file.take() {
            None => {
                tracing::trace!("Do nothing. The file hasn't been opened, so it hasn't been modified.");
            }
            Some(mut f) => {
                f.flush().unwrap_or_else(|e| {
                    error!(
                        "Cannot flush file content: {e}. ProxiedFile will be uploaded, but some content might be lost"
                    );
                });

                // TODO: I should wrap all of this somehow. Maybe into some `Drop` implementation of whatever is returned by the `create_side_task`.

                // Execute the side task and wait for it to finish
                self.drop_trigger
                    .take()
                    .unwrap()
                    .send((self.pcloud.take().unwrap(), self.file_id.clone(), self.local_filepath()))
                    .unwrap_or_else(|_| error!("Error sending the drop signal. Proxied file won't be uploaded."));

                // Wait for the upload work to finish
                match self.drop_completion.take().unwrap().blocking_recv() {
                    Ok(r) => {
                        if let Err(e) = r {
                            error!("Error executing the side task: {e}");
                        }
                    }
                    Err(e) => {
                        error!("Error waiting for drop task: {e}");
                    }
                }
            }
        }

        // The `File` is already dropped, and the content upload. We can remove the local file.
        std::fs::remove_file(self.local_filepath())
            .unwrap_or_else(|e| error!("Failed to remove the local file: {:?}: {e}", self.local_filepath()))
    }
}

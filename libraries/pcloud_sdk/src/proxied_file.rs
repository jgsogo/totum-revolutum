use crate::client::PCloudClient;
use crate::types::{FileID, FolderID};
use anyhow::Result;
use async_utils::SideTask;
use std::fs::File;
use std::future::Future;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use tempfile::{tempdir, TempDir};
use tracing::error;

type UploadFnType<PCloud> =
    Box<dyn FnOnce((PCloud, FileID, TempDir)) -> Pin<Box<dyn Future<Output = Result<(), TempDir>> + Send>> + Send>;

fn force_boxed<PCloud: PCloudClient + Send + 'static, T>(f: fn((PCloud, FileID, TempDir)) -> T) -> UploadFnType<PCloud>
where
    T: Future<Output = Result<(), TempDir>> + Send + 'static,
{
    Box::new(move |n| Box::pin(f(n)))
}

type UploadSideTaskType<PCloud> = SideTask<
    UploadFnType<PCloud>,
    Pin<Box<dyn Future<Output = Result<(), TempDir>> + Send + 'static>>,
    (PCloud, FileID, TempDir),
    Result<(), TempDir>,
>;

/// Keeps a local temporal copy of a remote file. The remote file is fetched (or created) as
/// soon as this object is instantiated, then you can use the local file as usual. When this
/// object is dropped, the local content is sent to the remote and the local file is removed.
pub struct ProxiedFile<PCloud: PCloudClient + Send + 'static> {
    /// The `PCloudClient` used to connect to the remote
    pcloud: Option<PCloud>,

    /// Remote `FileID`
    file_id: FileID,

    /// The temporal directory where the local file is located. It will be removed after dropped.
    temp_dir: Option<TempDir>,

    /// The `File` object. It's created on demand when the file is opened.
    local_file: Option<File>,

    /// Stores a task ([`upload`]) that will be executed when this object is dropped
    upload_on_drop: UploadSideTaskType<PCloud>,
}

impl<PCloud: PCloudClient + Send + 'static> ProxiedFile<PCloud> {
    fn get_local_filepath(directory: &Path) -> PathBuf {
        directory.join("proxied_file.tmp")
    }

    async fn upload_and_remove(args: (PCloud, FileID, TempDir)) -> Result<(), TempDir> {
        let (_, _, temp_dir) = args;

        // Execute upload
        let upload: Result<(), TempDir> = Ok(());

        // If upload fails, return the `TempDir` and let the user decide what to do
        match upload {
            Ok(ok) => Ok(ok),
            Err(_) => Err(temp_dir),
        }
    }

    /// Creates a local temporal file with the contents of the remote one (it will also create the
    /// remote one if it doesn't exist yet). Returns a tuple with the `ProxiedFile` object and
    /// a boolean indicating if the remote file was created or not.
    pub fn new(pcloud: PCloud, _folder_id: FolderID, _filename: &str) -> Result<(Self, bool)> {
        // Get or create the remote FileID
        let (file_id, created): (FileID, bool) = {
            // TODO: implement here
            (FileID(0), true)
        };

        // Return to the user
        let temp_dir = tempdir()?;
        let (upload_on_drop, _upload_receiver) = SideTask::new(force_boxed(Self::upload_and_remove), None)?;
        Ok((
            Self {
                pcloud: Some(pcloud),
                file_id,
                temp_dir: Some(temp_dir),
                local_file: None,
                upload_on_drop,
            },
            created,
        ))
    }

    /// Returns the full path to the local filename
    pub fn local_filepath(&self) -> PathBuf {
        Self::get_local_filepath(self.temp_dir.as_ref().unwrap().path())
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
        // If the file is opened, flush its content and close it.
        if let Some(mut f) = self.local_file.take() {
            f.flush().unwrap_or_else(|e| {
                error!("Cannot flush file content: {e}. ProxiedFile will be uploaded, but some content might be lost")
            });
        }

        // Trigger [`Self::upload_and_remove`] with the right arguments
        if let Err(e) = self.upload_on_drop.start(Some((
            self.pcloud.take().unwrap(),
            self.file_id.clone(),
            self.temp_dir.take().unwrap(),
        ))) {
            error!("Error starting the SideTask 'upload_and_remove': {e}")
        }
    }
}

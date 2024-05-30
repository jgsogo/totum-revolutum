use crate::client::PCloudClient;
use crate::handy::{GetFileLinkAndDownload, UploadToFileID};
use crate::methods::fileops::file_open::{FileOpenPath, Flags, GetFileOpen};
use crate::methods::folder::listfolder::GetListFolder;
use crate::methods::folder::ListFolderInput;
use crate::methods::streaming::getfilelink::GetFileLinkInput;
use crate::progress_bar::ProgressBarBuilder;
use crate::structures::Metadata;
use crate::types::{FileID, Folder, FolderID};
use anyhow::Result;
use async_utils::SideTask;
use camino::{Utf8Path, Utf8PathBuf};
use std::fs::File;
use std::future::Future;
use std::io::Write;
use std::path::PathBuf;
use std::pin::Pin;
use tempfile::{tempdir, TempDir};
use tokio::sync::oneshot::Receiver;
use tracing::{debug, error};

// TODO: Some 'Output' should arrive from outside. Remove this struct NoProgressBarBuilder
struct NoProgressBarBuilder;

impl ProgressBarBuilder for NoProgressBarBuilder {}

type UploadReturnType = Result<(), (TempDir, PathBuf)>;
type UploadFnType<PCloud> =
    Box<dyn FnOnce((PCloud, FileID, TempDir)) -> Pin<Box<dyn Future<Output = UploadReturnType> + Send>> + Send>;

fn force_boxed<PCloud: PCloudClient + Send + 'static, T>(f: fn((PCloud, FileID, TempDir)) -> T) -> UploadFnType<PCloud>
where
    T: Future<Output = UploadReturnType> + Send + 'static,
{
    Box::new(move |n| Box::pin(f(n)))
}

type UploadSideTaskType<PCloud> = SideTask<
    UploadFnType<PCloud>,
    Pin<Box<dyn Future<Output = UploadReturnType> + Send + 'static>>,
    (PCloud, FileID, TempDir),
    UploadReturnType,
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
    /// Return the path to the local file
    pub fn get_local_filepath(temp_dir: &TempDir) -> PathBuf {
        temp_dir.path().join("proxied_file.tmp")
    }

    /// Uploads file contents to the given [`FileID`]. Note that the file will be removed after
    /// this function call when [`TempDir`] goes out of scope, this is why in case of error it
    /// sends the [`TempDir`] to the consumer, so they can back up the file before it is removed.
    async fn upload_and_remove(args: (PCloud, FileID, TempDir)) -> UploadReturnType {
        let (pcloud, fileid, temp_dir) = args;

        // Execute upload
        let local_filepath = Self::get_local_filepath(&temp_dir);
        let r = pcloud
            .upload_to_fileid(
                Utf8Path::from_path(&local_filepath).expect("Every tmp path should be convertible to UTF8Path"),
                fileid,
            )
            .await;

        // If upload fails, return the `TempDir` and let the user decide what to do
        match r {
            Ok(ok) => Ok(ok),
            Err(_) => Err((temp_dir, local_filepath)),
        }
    }

    /// Creates a local temporal file with the contents of the remote one (it will also create the
    /// remote one if it doesn't exist yet). Returns a tuple with the [`ProxiedFile`] object,
    /// a boolean indicating if the remote file was created or not and the [`Receiver`] that will
    /// be called after the file is uploaded when the [`ProxiedFile`] is dropped.
    ///
    /// The [`Receiver`] can be ignored if the user is not interested on any error and the user
    /// guarantees that the runtime is not destroyed before the underlying task finishes after
    /// this [`ProxiedFile`] is dropped.
    pub async fn new(
        pcloud: PCloud,
        folder_id: FolderID,
        filename: &str,
    ) -> Result<(Self, bool, Receiver<UploadReturnType>)> {
        let (file_id, created): (FileID, bool) = {
            // Search the folder content for the file we are looking for
            let folder_content = pcloud
                .listfolder_with_filtermeta(
                    ListFolderInput::new(Folder::from(folder_id.clone())),
                    vec!["fileid", "name"],
                )
                .await?;
            let file_id = folder_content
                .metadata
                .contents
                .and_then(|files| {
                    files
                        .into_iter()
                        .filter_map(|m| match m {
                            Metadata::MetadataFile(m) => Some(m),
                            Metadata::MetadataFolder(_) => None,
                        })
                        .find(|f| f.common.name.as_ref().unwrap() == filename)
                })
                .map(|metadata| metadata.fileid);

            // If it exists, return. Otherwise, create the remote file
            match file_id {
                None => {
                    debug!("Remote file {filename} doesn't exist, it will be created");
                    let r = pcloud
                        .file_open(
                            Flags::O_CREAT,
                            FileOpenPath::FolderAndName(folder_id, filename.to_string()),
                        )
                        .await?;
                    (r.fileid, true)
                }
                Some(file_id) => (file_id, false),
            }
        };

        // Download the file to the local system
        let temp_dir = tempdir()?;
        let local_filepath = Utf8PathBuf::from_path_buf(Self::get_local_filepath(&temp_dir)).unwrap();
        debug!("File will be proxied in {}", local_filepath);
        pcloud
            .getfilelink_and_download(
                GetFileLinkInput::new(crate::types::File::FileID(file_id.clone())),
                &local_filepath,
                &NoProgressBarBuilder,
            )
            .await?;

        // Create side task (upload and remove on drop) and return to user
        let (upload_on_drop, upload_receiver) = SideTask::new(force_boxed(Self::upload_and_remove), None)?;
        Ok((
            Self {
                pcloud: Some(pcloud),
                file_id,
                temp_dir: Some(temp_dir),
                local_file: None,
                upload_on_drop,
            },
            created,
            upload_receiver,
        ))
    }

    /// Returns the full path to the local filename
    pub fn local_filepath(&self) -> PathBuf {
        Self::get_local_filepath(self.temp_dir.as_ref().unwrap())
    }

    /// Returns a reference to a [`std::fs::File`] object. The value is kept inside the
    /// `ProxiedFile` so it isn't possible to drop it while the file is opened.
    pub fn open(&mut self) -> Result<&File> {
        if self.local_file.is_none() {
            self.local_file = Some(File::open(self.local_filepath())?);
        }
        Ok(self.local_file.as_ref().unwrap())
    }
}

impl<PCloud: PCloudClient + Send + 'static> Drop for ProxiedFile<PCloud> {
    /// Before the `ProxiedFile` is dropped it runs a couple of actions:
    ///
    /// * the local file is flushed, in case someone has used [`ProxiedFile::open`] to get the
    ///   handle to the file.
    /// * the underlying task (upload and remove local file) is started asynchronously. The
    ///   [`Receiver`] returned from [`ProxiedFile::new`] can be used to get the result from
    ///   this task.
    ///
    /// Note that the runtime shouldn't be destroyed before the tasks finishes. This can be achieved
    /// by awaiting on the [`Receiver`].
    fn drop(&mut self) {
        // If the file is opened, flush its content and close it.
        if let Some(mut f) = self.local_file.take() {
            f.flush().unwrap_or_else(|e| {
                error!("Cannot flush file content: {e}. ProxiedFile will be uploaded, but some content might be lost")
            });
        }

        // Trigger `Self::upload_and_remove` with the right arguments
        if let Err(e) = self.upload_on_drop.start(Some((
            self.pcloud.take().unwrap(),
            self.file_id.clone(),
            self.temp_dir.take().unwrap(),
        ))) {
            error!("Error starting the SideTask 'upload_and_remove': {e}")
        }
    }
}

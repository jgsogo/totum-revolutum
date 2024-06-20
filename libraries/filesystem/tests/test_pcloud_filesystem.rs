use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use camino::Utf8PathBuf;

use filesystem::impls::FilesystemPCloud;
use filesystem::impls::PCLOUD_CHUNK_SIZE;
use filesystem::wrappers::AsyncFileDropImpl;
use filesystem::{DirectoryPathBuf, FilePathBuf, FilenameBuf, Filesystem};
use pcloud_sdk::access_token::OAuth2Token;
use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::mocks::server::PCloudServerMock;
use pcloud_sdk::types::{FolderID, RemotePath};

#[tokio::test]
async fn test_create_write_read_in_root_folder() -> Result<()> {
    let root_path = Utf8PathBuf::from("/the/root/path");
    let mut server = PCloudServerMock::default();
    let mut fs = {
        let server_token = server.token();
        let root_folder_mock = {
            let mut qparams = HashMap::new();
            qparams.insert("path", "/the/root/path");
            qparams.insert("filtermeta", "folderid,fileid");
            qparams.insert("access_token", server_token.access_token());
            server.listfolder_mock(qparams, "{\"result\": 0, \"metadata\": {\"folderid\": 1234}}")
        };

        let pcloud = PCloudClientImpl::new(server_token, false);
        let root_path_remote = RemotePath::try_from(root_path.as_path())?;
        let r = FilesystemPCloud::new(root_path_remote, pcloud).await?;
        root_folder_mock.assert();

        // Use AsyncFileDropImpl so the [`File::sync_all`] method is called when created files are dropped
        AsyncFileDropImpl::new_call_sync_all(r)
    };

    // Add mock so we can create a file
    let folderid = FolderID::new(1234);
    let name = String::from("myfile.txt");
    let write_bytes = 100;
    let content = "the content".as_bytes().to_vec();
    let (create, open, open_with_path, write, read, read_eof, close) = server.fileops_create_with_folder_and_name(
        folderid.clone(),
        &name,
        &root_path,
        write_bytes,
        content.clone(),
        PCLOUD_CHUNK_SIZE,
    );

    let p = FilePathBuf::new(DirectoryPathBuf::root(), FilenameBuf::from_str(&name).unwrap());
    // Create and write
    let rx = {
        let (mut f, rx) = fs.create(&p).await?;
        f.write_all(&content).await?;
        rx
    };
    if let Some(rx) = rx {
        rx.await??;
    }

    // Open and read
    {
        let (mut file, _) = fs.open(&p).await?;
        let mut content_read = Vec::new();
        file.read_to_end(&mut content_read).await?;
        assert_eq!(content, content_read);
    }

    create.assert();
    write.assert();
    open_with_path.assert();
    read.assert();
    read_eof.assert();
    close.assert_hits(1); // FIXME: There should be two hits here.
    open.assert_hits(0);
    Ok(())
}

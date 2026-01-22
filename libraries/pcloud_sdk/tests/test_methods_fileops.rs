use camino::Utf8Path;

use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::methods::fileops::file_close::GetFileClose;
use pcloud_sdk::methods::fileops::file_open::GetFileOpen;
use pcloud_sdk::methods::fileops::file_open::{FileOpenPath, Flags};
use pcloud_sdk::methods::fileops::file_read::GetFileRead;
use pcloud_sdk::methods::fileops::file_write::PostFileWrite;
use pcloud_sdk::mocks::server::PCloudServerMock;
use pcloud_sdk::types::FolderID;
use pcloud_sdk::Result;

#[tokio::test]
async fn test_fileops() -> Result<()> {
    let mut server = PCloudServerMock::default();
    //let userinfo_mock = server.fileops_();
    let oauth2_token = server.token();

    let pcloud = PCloudClientImpl::new(oauth2_token, false);

    // Add mock so we can create a file
    let folderid = FolderID::new(42);
    let root_path = Utf8Path::new("the/root/path");
    let name = String::from("myfile.txt");
    let write_bytes = 100;
    let content = "the content".as_bytes().to_vec();
    let chunk_size = 100;
    let (create, open, open_with_path, write, read, read_eof, close) = server.fileops_create_with_folder_and_name(
        folderid.clone(),
        &name,
        root_path,
        write_bytes,
        content.clone(),
        chunk_size.clone(),
    );

    // Open + write + close
    let fileid = {
        let fd = pcloud
            .file_open(
                Flags::O_CREAT | Flags::O_WRITE | Flags::O_TRUNC | Flags::O_APPEND,
                FileOpenPath::FolderAndName(folderid.clone(), name.clone()),
            )
            .await?;
        create.assert();

        let bytes_count = pcloud
            .file_write(fd.fd.clone(), &"eaeaeaea".as_bytes().to_vec())
            .await?;
        assert_eq!(bytes_count.bytes, write_bytes);
        write.assert();

        pcloud.file_close(fd.fd.clone()).await?;
        close.assert();

        open_with_path.assert_calls(0);
        fd.fileid
    };

    // Open + read + close
    {
        let fd = pcloud
            .file_open(Flags::empty(), FileOpenPath::File(fileid.into()))
            .await?;
        open.assert();

        let r = pcloud.file_read(fd.fd.clone(), chunk_size as u64).await?;
        assert_eq!(String::from_utf8_lossy(&*r.bytes), String::from_utf8_lossy(&*content));
        read.assert();

        let remain_count = chunk_size - content.len();
        let r = pcloud.file_read(fd.fd.clone(), remain_count as u64).await?;
        assert_eq!(String::from_utf8_lossy(&*r.bytes), "");
        read_eof.assert();

        pcloud.file_close(fd.fd.clone()).await?;
        close.assert_calls(2);
    }

    Ok(())
}

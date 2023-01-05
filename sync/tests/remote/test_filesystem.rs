use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::data::oauth2token::OAuth2Token;
use pcloud_sdk::mocks::server::PCloudServerMock;
use pcloud_sdk::types::FolderID;
use pcloud_sync::diff::filesystem::Filesystem;
use pcloud_sync::remote::filesystem::FilesystemPCloud;

#[tokio::test]
async fn test_create_write_read_in_root_folder() -> Result<()> {
    let root_path = Path::new("the/root/path");
    let mut server = PCloudServerMock::default();
    let fs = {
        let server_token = server.token();
        let root_folder_mock = {
            let mut qparams = HashMap::new();
            qparams.insert("path", "the/root/path");
            qparams.insert("filtermeta", "folderid,id");
            qparams.insert("access_token", server_token.access_token());
            server.listfolder_mock(qparams, "{\"result\": 0, \"metadata\": {\"folderid\": 1234}}")
        };

        let pcloud = HttpClient::new(server_token, false);
        let r = FilesystemPCloud::new(root_path, pcloud).await?;
        root_folder_mock.assert();
        r
    };

    // Add mock so we can create a file
    let folderid = FolderID(1234);
    let name = String::from("myfile.txt");
    let write_bytes = 100;
    let content = "the content".as_bytes().to_vec();
    let (create, open, open_with_path, write, read, close) =
        server.fileops_create_with_folder_and_name(folderid.clone(), &name, &root_path, write_bytes, content.clone());

    let p = PathBuf::from(name.clone());
    // Create and write
    {
        let mut f = fs.create(&*p).await?;
        f.write_all(&content).await?;
    }

    // Open and read
    {
        let mut file = fs.open(&*p).await?;
        let mut content_read = Vec::new();
        file.read_to_end(&mut content_read).await?;
        assert_eq!(content, content_read);
    }

    create.assert();
    write.assert();
    open_with_path.assert();
    read.assert();

    close.assert_hits(0);
    open.assert_hits(0);
    Ok(())
}

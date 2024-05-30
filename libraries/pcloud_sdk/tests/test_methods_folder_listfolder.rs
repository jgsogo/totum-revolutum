use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;

use pcloud_sdk::access_token::OAuth2Token;
use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::mocks::server::PCloudServerMock;
use pcloud_sdk::types::{Folder, FolderID};

#[tokio::test]
async fn test_listfolder() -> Result<()> {
    let server = PCloudServerMock::default();
    let server_token = server.token();
    let listfolder_mock = {
        let mut qparams = HashMap::new();
        qparams.insert("path", "/the/root/path");
        qparams.insert("access_token", server_token.access_token());
        server.listfolder_mock(qparams, "{\"result\": 0, \"metadata\": {\"folderid\": 1234}}")
    };

    let pcloud = PCloudClientImpl::new(server_token, false);
    let input = ListFolderInput::new(Folder::from_str("path:/the/root/path")?);
    let data = pcloud.listfolder(input).await?;

    assert_eq!(data.metadata.folderid, FolderID::new(1234));

    listfolder_mock.assert();
    Ok(())
}

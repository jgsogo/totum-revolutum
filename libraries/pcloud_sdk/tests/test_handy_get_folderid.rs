use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;

use pcloud_sdk::access_token::OAuth2Token;
use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::handy::GetFolderID;
use pcloud_sdk::mocks::server::PCloudServerMock;
use pcloud_sdk::types::{FolderID, RemotePath};

#[tokio::test]
async fn test_get_folderid() -> Result<()> {
    let server = PCloudServerMock::default();
    let server_token = server.token();
    let listfolder_mock = {
        let mut qparams = HashMap::new();
        qparams.insert("path", "/the/path");
        qparams.insert("filtermeta", "folderid,fileid");
        qparams.insert("access_token", server_token.access_token());
        server.listfolder_mock(qparams, "{\"result\": 0, \"metadata\": {\"folderid\": 1234}}")
    };

    let pcloud = PCloudClientImpl::new(server_token, false);
    let remote_path = RemotePath::from_str("path:/the/path")?;
    let data = pcloud.get_folderid(&remote_path).await?;

    assert_eq!(data, FolderID(1234));

    listfolder_mock.assert();
    Ok(())
}

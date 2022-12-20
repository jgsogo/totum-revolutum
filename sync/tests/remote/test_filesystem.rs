use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::data::oauth2token::OAuth2Token;
use pcloud_sdk::mocks::server::PCloudServerMock;
use pcloud_sync::diff::filesystem::Filesystem;
use pcloud_sync::remote::filesystem::FilesystemPCloud;

// fn oauth2_token() -> impl OAuth2Token {
//     match env::var("TESTING_PCLOUD_TOKEN") {
//         Ok(v) => {
//             // If the variable is available, then we are using actual pcloud
//         }
//         Err(e) => {
//             // If it is not available, we use a mocked server and a temporal filesystem
//
//             // Start a lightweight mock server.
//             let server = MockServer::start();
//
//             // Create a mock on the server.
//             let userinfo_mock = server.mock(|when, then| {
//                 when.method(GET).path("/userinfo").query_param("access_token", "token");
//
//                 let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
//                 let userinfo_json = Path::new(&manifest_dir)
//                     .join("resources")
//                     .join("testdata")
//                     .join("userinfo.json");
//                 then.status(200)
//                     .header("content-type", "application/json; charset=UTF-8")
//                     .body_from_file(userinfo_json.to_str().unwrap());
//             });
//
//             OAuth2TokenMock::new(&format!("{}:{}", server.host(), server.port()), "token")
//         }
//     }
// }

#[tokio::test]
async fn test_create_write_read() -> Result<()> {
    let server = PCloudServerMock::new();
    let fs = {
        let server_token = server.token();
        let root_folder_mock = {
            let mut qparams = HashMap::new();
            qparams.insert("path", "the/root/path");
            qparams.insert("filtermeta", "folderid");
            qparams.insert("access_token", server_token.access_token());
            server.listfolder_mock(qparams, "{\"result\": 0, \"metadata\": {\"folderid\": 1234}}")
        };

        let pcloud = HttpClient::new(server_token, false);
        let r = FilesystemPCloud::new(Path::new("the/root/path"), pcloud).await?;
        root_folder_mock.assert();
        r
    };

    let filepath = Path::new("filepath");
    let content: Vec<u8> = b"Hello, world!".to_vec();

    // Create and write
    {
        let mut f = fs.create(&filepath).await?;
        f.write_all(&content).await?;
    }

    // Open and read
    // {
    //     let mut file = fs.open(&filepath).await?;
    //     let mut content_read = Vec::new();
    //     file.read_to_end(&mut content_read).await?;
    //     assert_eq!(content, content_read);
    // }

    Ok(())
}

use std::path::Path;

use anyhow::Result;

use pcloud_sdk::data;
use pcloud_sdk::data::oauth2token::OAuth2TokenImpl;
use pcloud_sdk::handy::GetCreateFolderIfNotExistsAll;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;
use pcloud_sync::filesystem::copy::copy;
use pcloud_sync::filesystem::Filesystem;
use pcloud_sync::remote::filesystem::FilesystemPCloud;

#[tokio::main]
async fn main() -> Result<()> {
    println!("==== Backup file===");

    let fs = {
        let pcloud = {
            let app = data::app_client_data::AppClientData::read_from_file("secrets/app.json").unwrap();
            let addr = ([127, 0, 0, 1], 3000).into(); // But this address needs to be configured in the app
            pcloud_sdk::client::HttpClient::<OAuth2TokenImpl>::authorize(app, addr).await?
        };

        let userinfo = pcloud.userinfo().await?;
        println!("{:#?}", userinfo);

        let base_path = Path::new("/backup_file");
        pcloud.createfolderifnotexists_all(&base_path).await?;
        FilesystemPCloud::new(&base_path, pcloud).await?
    };

    // Creates a file in the origin
    let origin = Path::new("origin.txt");
    {
        let mut origin_file = fs.create(&origin).await?;
        origin_file
            .write_all(
                b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
            sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
            Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris \
            nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in \
            reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla \
            pariatur. Excepteur sint occaecat cupidatat non proident, sunt in \
            culpa qui officia deserunt mollit anim id est laborum.",
            )
            .await?;
    }

    // Now copies origin to target
    let target = Path::new("target.txt");
    copy(&fs, &fs, &origin, &target, false).await?;

    // And let's read target
    let data = {
        let mut target_file = fs.open(target).await?;
        let mut data = Vec::new();
        target_file.read_to_end(&mut data).await?;
        data
    };

    let read_content = String::from_utf8_lossy(&*data);
    println!("Content: {}", read_content);

    Ok(())
}

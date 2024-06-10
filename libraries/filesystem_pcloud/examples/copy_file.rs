use std::str::FromStr;

use anyhow::Result;
use camino::Utf8Path;

use filesystem::actions::copy_file;
use filesystem::{FilesystemRead, FilesystemWrite};
use filesystem_pcloud::FilesystemPCloud;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;
use pcloud_sdk::methods::oauth2;
use pcloud_sdk::types::RemotePath;

#[tokio::main]
async fn main() -> Result<()> {
    println!("==== Backup file===");

    let fs = {
        let pcloud = {
            let app = oauth2::AppClientData::read_from_file("secrets/app.json").unwrap();
            let addr = ([127, 0, 0, 1], 3000).into(); // But this address needs to be configured in the app
            pcloud_sdk::client::PCloudClientImpl::<oauth2::OAuth2TokenImpl>::authorize(app, addr).await?
        };

        let userinfo = pcloud.userinfo().await?;
        println!("{:#?}", userinfo);

        let root_path = RemotePath::from_str("path:/backup_file")?;
        FilesystemPCloud::new(root_path, pcloud).await?
    };

    // Creates a file in the origin
    let origin = Utf8Path::new("origin.txt");
    let rx = {
        println!("Creates file in origin: {origin}");
        let (mut origin_file, rx) = fs.create(&origin).await?;
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
        rx
    };
    let _ = rx.await?;

    // Now copies origin to target
    let target = Utf8Path::new("target.txt");
    println!("Copies from origin '{origin}' to target '{target}'");
    copy_file(&fs, &fs, &origin, &target, false).await?;

    // And let's read target
    let data = {
        println!("Reads content from target '{target}'");
        let mut target_file = fs.open(target).await?;
        let mut data = Vec::new();
        target_file.read_to_end(&mut data).await?;
        data
    };

    let read_content = String::from_utf8_lossy(&*data);
    println!("Content: {}", read_content);

    Ok(())
}

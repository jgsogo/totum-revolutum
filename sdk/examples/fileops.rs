use pcloud_sdk::data;
use pcloud_sdk::data::oauth2token::OAuth2TokenImpl;
use pcloud_sdk::methods::fileops::file_close::GetFileClose;
use pcloud_sdk::methods::fileops::file_open::{FileOpenPath, Flags, GetFileOpen};
use pcloud_sdk::methods::fileops::file_read::GetFileRead;
use pcloud_sdk::methods::fileops::file_write::PostFileWrite;
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::methods::general::getapiserver::GetAPIServer;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;
use pcloud_sdk::types::FolderID;
use serde::de::Unexpected::Str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = data::app_client_data::AppClientData::read_from_file("secrets/app.json").unwrap();
    let addr = ([127, 0, 0, 1], 3000).into(); // But this address needs to be configured in the app
    let pcloud = pcloud_sdk::client::HttpClient::<OAuth2TokenImpl>::authorize(app, addr).await?;

    let userinfo = pcloud.userinfo().await?;
    println!("{:#?}", userinfo);

    let listfolder_input = ListFolderInput::new_from_path(Some("/".into()));
    let listfolder = pcloud.listfolder(&listfolder_input).await?;
    let folderid = listfolder.metadata.folderid.unwrap();
    println!("{}", folderid);

    // Open + write + close
    let fileid = {
        let fd = pcloud
            .file_open(
                Flags::O_CREAT | Flags::O_WRITE | Flags::O_TRUNC | Flags::O_APPEND,
                FileOpenPath::FolderAndName(folderid.clone(), "nested/name2.txt".to_string()),
            )
            .await?;
        println!("File is opened with descriptor {}", fd.fd);

        let bytes_count = pcloud.file_write(fd.fd, &mut "eaeaeaea".as_bytes().to_vec()).await?;
        println!("wrote {} bytes", bytes_count.bytes);

        pcloud.file_close(fd.fd).await?;
        println!("File is closed");

        fd.fileid
    };

    // Open + read + close
    {
        let fd = pcloud.file_open(Flags::empty(), FileOpenPath::FileID(fileid)).await?;
        println!("File is opened with descriptor {}", fd.fd);

        let r = pcloud.file_read(fd.fd, 100).await?;
        let content = String::from_utf8_lossy(&*r.bytes);
        println!("Content: {}", content);

        pcloud.file_close(fd.fd).await?;
        println!("File is closed");
    }

    Ok(())
}

use pcloud_sdk::data;
use pcloud_sdk::data::oauth2token::OAuth2TokenImpl;
use pcloud_sdk::methods::fileops::file_close::GetFileClose;
use pcloud_sdk::methods::fileops::file_open::{FileOpenPath, Flags, GetFileOpen};
use pcloud_sdk::methods::fileops::file_read::GetFileRead;
use pcloud_sdk::methods::fileops::file_write::PostFileWrite;
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;

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

    let content = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
    tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation \
    ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
    voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non \
    proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
        .to_vec();

    // Open + write + close
    let fileid = {
        let fd = pcloud
            .file_open(
                Flags::O_CREAT | Flags::O_WRITE | Flags::O_TRUNC | Flags::O_APPEND,
                FileOpenPath::FolderAndName(folderid.clone(), "name2.txt".to_string()),
            )
            .await?;
        println!("File is opened with descriptor {}", fd.fd);

        let bytes_count = pcloud.file_write(fd.fd, &mut content.clone()).await?;
        println!("wrote {} bytes", bytes_count.bytes);

        pcloud.file_close(fd.fd).await?;
        println!("File is closed");

        fd.fileid
    };

    // Open + read + close
    {
        let fd = pcloud.file_open(Flags::empty(), FileOpenPath::FileID(fileid)).await?;
        println!("File is opened with descriptor {}", fd.fd);

        let buffer_len: u64 = 100;
        assert!(buffer_len < content.len() as u64, "Ensure we read several chunks");
        let mut content_read = Vec::<u8>::new();
        loop {
            let mut r = pcloud.file_read(fd.fd, buffer_len).await?;
            let read_len = r.bytes.len() as u64;
            let read_content = String::from_utf8_lossy(&*r.bytes);
            println!("Read iteration (len {read_len}): {read_content}");
            content_read.append(&mut r.bytes);
            if read_len < buffer_len {
                println!("Read exhausted");
                break;
            }
        }
        // let content = String::from_utf8_lossy(&*r.bytes);
        let read_content = String::from_utf8_lossy(&*content_read);
        println!("Content: {}", read_content);
        assert_eq!(read_content, String::from_utf8_lossy(&*content));

        pcloud.file_close(fd.fd).await?;
        println!("File is closed");
    }

    Ok(())
}

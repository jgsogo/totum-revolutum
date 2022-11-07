use pcloud_sdk::folder::listfolder::GetListFolder;
use pcloud_sdk::folder::ListFolderInput;
use pcloud_sdk::general::getapiserver::GetAPIServer;
use pcloud_sdk::general::userinfo::GetUserInfo;
use pcloud_sdk::methods::oauth2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = oauth2::AppClientData::read_from_file("secrets/app.json").unwrap();
    let addr = ([127, 0, 0, 1], 3000).into(); // But this address needs to be configured in the app
    let pcloud = pcloud_sdk::client::HttpClient::authorize(app, addr).await?;

    let userinfo = pcloud.userinfo().await?;
    println!("{:#?}", userinfo);

    let apiserver = pcloud.getapiserver().await?;
    println!("{:#?}", apiserver);

    let listfolder_input = ListFolderInput::new_from_path("/");
    let listfolder = pcloud.listfolder(&listfolder_input).await?;
    println!("{:#?}", listfolder);

    Ok(())
}

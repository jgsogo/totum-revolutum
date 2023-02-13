use anyhow::Result;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;

pub async fn handle(pcloud: HttpClient<OAuth2TokenImpl>) -> Result<()> {
    let userinfo = pcloud.userinfo().await?;
    println!("{:#?}", userinfo);

    Ok(())
}

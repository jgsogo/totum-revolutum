use anyhow::Result;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;

use crate::output::{Print, PrintVariant};

pub async fn handle(pcloud: HttpClient<OAuth2TokenImpl>, output: &PrintVariant) -> Result<()> {
    let userinfo = pcloud.userinfo().await?;
    output.user_info(&userinfo)
}

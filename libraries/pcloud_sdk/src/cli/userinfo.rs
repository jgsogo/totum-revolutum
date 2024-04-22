use anyhow::Result;

use crate::CliParams;
use pcloud_sdk::client::PCloudClientImpl;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;

use crate::output::{Print, PrintVariant};

pub async fn handle(
    pcloud: PCloudClientImpl<OAuth2TokenImpl>,
    output: &PrintVariant,
    _cli_params: CliParams,
) -> Result<()> {
    let userinfo = pcloud.userinfo().await?;
    output.user_info(&userinfo)
}

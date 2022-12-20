use anyhow::Result;
use time::macros::datetime;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;
use pcloud_sdk::mocks::server::PCloudServerMock;

#[tokio::test]
async fn test_userinfo_get() -> Result<()> {
    let server = PCloudServerMock::new();
    let userinfo_mock = server.userinfo_mock();
    let oauth2_token = server.token();

    let pcloud = HttpClient::new(oauth2_token, false);
    let data = pcloud.userinfo().await?;

    assert_eq!(data.email, "pcloud@pcloud.com".to_string());
    assert_eq!(data.emailverified, true);
    assert_eq!(data.registered, datetime!(2013-11-18 15:32:05 UTC));
    assert_eq!(data.premium, false);
    assert_eq!(data.premiumexpires, None);
    assert_eq!(data.quota, 1000);
    assert_eq!(data.usedquota, 500);
    assert_eq!(data.language, "en".to_string());

    userinfo_mock.assert();
    Ok(())
}

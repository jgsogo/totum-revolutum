use finances_app_lib::models::Snapshot;
use serde_json::json;

mod common;
use common::call_it;

#[test]
fn test_account_snapshot_latest() {
    let webview = common::webview();

    {
        let body = json!({ "pk": 0i64 });
        let r = call_it::<Option<Snapshot>>(&webview, "get_account_snapshot_latest".to_string(), body);

        assert!(r.is_ok());
        let snapshot = r.unwrap();
        assert!(snapshot.is_some());
        let snapshot = snapshot.unwrap();
        assert_eq!(snapshot.account_id, 0i64);
    }

    {
        let body = json!({ "pk": -2i64 });
        let r = call_it::<Option<Snapshot>>(&webview, "get_account_snapshot_latest".to_string(), body);

        // We filter using the account-pk, it doesn't check if the account exists. This is the reason
        // why it returns an empty value instead of an error
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_none());
    }
}

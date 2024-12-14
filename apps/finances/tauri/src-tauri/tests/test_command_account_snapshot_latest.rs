use finances_app_lib::models::Snapshot;
use serde_json::json;

mod common;
use common::call_it;

#[test]
fn test_account_snapshot_latest() {
    let webview = common::webview();

    // Account with snapshots
    {
        let body = json!({ "pk": 0i64 });
        let r = call_it::<Option<Snapshot>>(&webview, "get_account_snapshot_latest".to_string(), body);

        assert!(r.is_ok());
        let snapshot = r.unwrap();
        assert!(snapshot.is_some());
        let snapshot = snapshot.unwrap();
        assert_eq!(snapshot.account_id, 0i64);
    }

    // Account that doesn't exists
    {
        let body = json!({ "pk": -2i64 });
        let r = call_it::<Option<Snapshot>>(&webview, "get_account_snapshot_latest".to_string(), body);

        assert!(r.is_err());
        assert_eq!(r.unwrap_err(), "Error loading account: Record not found");
    }
}

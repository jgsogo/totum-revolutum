use finances_app_lib::models::Snapshot;
use serde_json::json;

mod common;
use common::call_it;

#[test]
fn test_account_snapshots() {
    let webview = common::webview();

    // Account with snapshots
    {
        let body = json!({ "pk": 0i32 });
        let r = call_it::<Vec<Snapshot>>(&webview, "get_account_snapshots".to_string(), body);

        assert!(r.is_ok());
        let snapshots = r.unwrap();
        assert_eq!(snapshots.len(), 2);
        let latest = snapshots.get(0).unwrap();
        let next = snapshots.get(1).unwrap();
        assert!(latest.date_value > next.date_value);
    }

    // Account that doesn't exist
    {
        let body = json!({ "pk": -2i32 });
        let r = call_it::<Vec<Snapshot>>(&webview, "get_account_snapshots".to_string(), body);

        assert!(r.is_err());
        assert_eq!(r.unwrap_err(), "Error loading account: Record not found");
    }
}

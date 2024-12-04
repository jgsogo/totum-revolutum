use finances_app_lib::models::Snapshot;
use serde_json::json;

mod common;
use common::call_it;

#[test]
fn test_snapshot() {
    let webview = common::webview();

    /****
    Non-numerable account
    ***/
    let account_id = 1i32;
    {
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "get_account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 0);
    }

    // Snapshot (non-numerable)
    {
        let body = json!({
            "accountPk": account_id,
            "dateValue": "2024-11-30",
            "amount": Some(100f32),
        });

        let r = call_it::<usize>(&webview, "create_snapshot".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());

        // Now we have one more snapshot
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "get_account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 1);
    }

    /****
    Numerable account
    ***/
    let account_id = 8i32;
    {
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "get_account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 0);
    }

    // Snapshot (non-numerable)
    {
        let body = json!({
            "accountPk": account_id,
            "dateValue": "2024-11-30",
            "amount": Some(300f32),
            "quantity": Some(3f32),
            "unitValue": Some(100f32),

        });

        let r = call_it::<usize>(&webview, "create_snapshot".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());

        // Now we have one more snapshot
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "get_account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 1);
    }
}

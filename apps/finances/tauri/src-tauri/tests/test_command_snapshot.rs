use finances_app_lib::models::{NewAmount, NewSnapshot, Snapshot};
use serde_json::json;

mod common;
use common::call_it;

#[test]
fn test_snapshot() {
    let webview = common::webview();

    /****
    Non-numerable account
    ***/
    let account_id = 1i64;
    {
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "get_account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 0);
    }

    // Snapshot (non-numerable)
    {
        let body = {
            let new_snapshot = NewSnapshot {
                account_pk: account_id,
                date_value: "2024-11-30".to_string(),
                amount: NewAmount {
                    amount: Some(100f32),
                    quantity: None,
                    unit_value: None,
                },
            };
            json!({
                "snapshot": serde_json::to_value(new_snapshot).unwrap(),
            })
        };

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
    let account_id = 8i64;
    {
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "get_account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 0);
    }

    // Snapshot (numerable)
    {
        let body = {
            let new_snapshot = NewSnapshot {
                account_pk: account_id,
                date_value: "2024-11-30".to_string(),
                amount: NewAmount {
                    amount: None,
                    quantity: Some(3f32),
                    unit_value: Some(100f32),
                },
            };
            json!({
                "snapshot": serde_json::to_value(new_snapshot).unwrap(),
            })
        };

        let r = call_it::<usize>(&webview, "create_snapshot".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());

        // Now we have one more snapshot
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "get_account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 1);
    }
}

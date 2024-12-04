use finances_app_lib::models::Movement;
use serde_json::json;

mod common;
use common::call_it;

#[test]
fn test_account_movements() {
    let webview = common::webview();

    {
        let body = json!({ "pk": 0i32 });
        let r = call_it::<Vec<Movement>>(&webview, "get_account_movements".to_string(), body);

        assert!(r.is_ok());
        let movements = r.unwrap();
        assert_eq!(movements.len(), 2);
        let latest = movements.get(0).unwrap();
        let next = movements.get(1).unwrap();
        assert!(latest.date_value > next.date_value);
    }

    {
        let body = json!({ "pk": -2i32 });
        let r = call_it::<Vec<Movement>>(&webview, "get_account_movements".to_string(), body);

        // We filter using the account-pk, it doesn't check if the account exists. This is the reason
        // why it returns an empty vector instead of an error
        assert!(r.is_ok());
        assert_eq!(r.unwrap().len(), 0);
    }
}

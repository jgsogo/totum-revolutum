use finances_app_lib::models::Account;
use serde_json::{json, Value};

mod common;
use common::call_it;

#[test]
fn test_account_detail() -> Result<(), Value> {
    let webview = common::webview();

    {
        let body = json!({ "pk": 0i32 });
        let r = call_it::<Account>(&webview, "account_detail".to_string(), body)?;
        assert_eq!(r.name, "Gastos compartidos");
    }

    {
        let body = json!({ "pk": -2i32 });
        let r = call_it::<Account>(&webview, "account_detail".to_string(), body);

        assert!(r.is_err());
        let r = r.unwrap_err();
        assert_eq!(r.as_str().unwrap(), "Error loading account: Record not found");
    }
    Ok(())
}

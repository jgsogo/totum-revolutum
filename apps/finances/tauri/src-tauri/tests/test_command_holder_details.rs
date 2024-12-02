use finances_app_lib::models::Holder;
use serde_json::{json, Value};

mod common;
use common::call_it;

#[test]
fn test_holders() -> Result<(), Value> {
    let webview = common::webview();

    {
        let body = json!({"pk": 0i64});
        let r = call_it::<Holder>(&webview, "holder_details".to_string(), body)?;
        assert_eq!(r.name, "holder0");
        assert!(!r.is_company);
    }

    Ok(())
}

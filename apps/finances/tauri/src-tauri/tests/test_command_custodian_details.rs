use finances_app_lib::models::Custodian;
use serde_json::{json, Value};

mod common;
use common::call_it;

#[test]
fn test_custodians() -> Result<(), Value> {
    let webview = common::webview();

    {
        let body = json!({"pk": 0i64});
        let r = call_it::<Custodian>(&webview, "get_custodian_details".to_string(), body)?;
        assert_eq!(r.name, "custodian0");
    }

    Ok(())
}

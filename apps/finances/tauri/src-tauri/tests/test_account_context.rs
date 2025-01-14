use serde_json::json;

mod common;
use common::call_it_proto;
use finances_app_models::AccountContext;

#[test]
fn test_holder_context() {
    let webview = common::webview();

    {
        let body = json!({"accountPk": 0i64});
        let r = call_it_proto::<AccountContext>(&webview, "get_account_context".to_string(), body);

        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let account_context = r.unwrap();

        let snapshots = account_context.snapshots();
        assert_eq!(snapshots.len(), 2);
    }
}

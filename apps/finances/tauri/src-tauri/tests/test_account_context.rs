use serde_json::json;

mod common;
use common::call_command;
use finances_app_models::AccountContext;

#[test]
fn test_account_context() {
    let webview = common::webview();

    {
        let body = json!({"accountPk": 0i64});
        let r = call_command(&webview, "get_account_context", body.into());

        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let account_context: AccountContext = r.unwrap().try_into_proto().unwrap();

        let snapshots = account_context.snapshots();
        assert_eq!(snapshots.len(), 2);
    }
}

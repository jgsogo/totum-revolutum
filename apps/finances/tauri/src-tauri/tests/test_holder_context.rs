use serde_json::json;

mod common;
use common::call_command;
use finances_app_models::HolderContext;

#[test]
fn test_holder_context() {
    let webview = common::webview();

    {
        let body = json!({"holderPk": 0i64});
        let r = call_command(&webview, "get_holder_context", body.into());

        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let _holder_context: HolderContext = r.unwrap().try_into_proto().unwrap();
    }
}

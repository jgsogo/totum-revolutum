use serde_json::json;

mod common;
use common::call_command;
use finances_app_models::MainContext;

#[test]
fn test_app_state() {
    let webview = common::webview();

    {
        let body = json!({});
        let r = call_command(&webview, "get_main_context", body.into());

        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        let _main_context: MainContext = r.unwrap().try_into_proto().unwrap();
    }
}

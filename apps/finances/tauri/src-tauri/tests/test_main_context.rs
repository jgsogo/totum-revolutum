use serde_json::json;

mod common;
use common::call_it_proto;
use finances_app_models::MainContext;

#[test]
fn test_app_state() {
    let webview = common::webview();

    {
        let body = json!({});
        let r = call_it_proto::<MainContext>(&webview, "get_main_context".to_string(), body);

        assert!(r.is_ok());
        let _main_context = r.unwrap();
    }
}

use serde_json::json;

mod common;
use common::call_it_proto;
use finances_app_models::HolderContext;

#[test]
fn test_holder_context() {
    let webview = common::webview();

    {
        let body = json!({"holderPk": 0i64});
        let r = call_it_proto::<HolderContext>(&webview, "get_holder_context".to_string(), body);

        assert!(r.is_ok());
        let _holder_context = r.unwrap();
    }
}

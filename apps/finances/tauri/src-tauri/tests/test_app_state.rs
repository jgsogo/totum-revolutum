use serde_json::json;

mod common;
use common::call_it_proto;
use finances_app_models::AppState;

#[test]
fn test_app_state() {
    let webview = common::webview();

    {
        let body = json!({});
        let r = call_it_proto::<AppState>(&webview, "get_app_state".to_string(), body);

        assert!(r.is_ok());
        let app_state = r.unwrap();
        assert_eq!(app_state.base_ccy(), "USD");
        assert_eq!(app_state.postgres_url(), "postgres_url");
    }
}

use serde_json::json;

mod common;
use common::call_command;
use finances_app_models::{google_type, AppState};

#[test]
fn test_app_state() {
    let webview = common::webview();

    {
        let body = json!({});
        let r = call_command(&webview, "get_app_state", body.into());

        assert!(r.is_ok());
        let app_state: AppState = r.unwrap().try_into_proto().unwrap();
        assert_eq!(app_state.base_ccy().unwrap(), google_type::CurrencyCode::USD);
        assert_eq!(app_state.postgres_url().unwrap(), "postgres_url");
    }
}

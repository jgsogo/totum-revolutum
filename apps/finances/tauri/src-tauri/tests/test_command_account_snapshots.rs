use finances_accounts::test_utils::establish_connection;
use finances_app_lib::models::Snapshot;
use finances_app_lib::state::AppState;
use serde_json::{json, Value};
use tauri::{test::MockRuntime, Manager, WebviewWindow};

fn call_it(webview: &WebviewWindow<MockRuntime>, body: Value) -> Result<Vec<Snapshot>, Value> {
    tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: "account_snapshots".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        },
    )
    .map(|b: tauri::ipc::InvokeResponseBody| b.deserialize::<Vec<Snapshot>>().unwrap())
}

#[test]
fn test_account_snapshots() {
    let pool = establish_connection();
    let app_state = AppState::new(
        "postgres_url".to_string(),
        "base_url".to_string(),
        "media_url".to_string(),
        "static_url".to_string(),
    );

    let app = finances_app_lib::create_app(tauri::test::mock_builder(), pool.clone(), app_state);
    app.manage(pool); // FIXME: The `.manage` inside `create_app` is not working for the mock.
    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    {
        let body = json!({ "pk": 0i32 });
        let r = call_it(&webview, body);

        assert!(r.is_ok());
        let snapshots = r.unwrap();
        assert_eq!(snapshots.len(), 2);
        let latest = snapshots.get(0).unwrap();
        let next = snapshots.get(1).unwrap();
        assert!(latest.date_value > next.date_value);
    }

    {
        let body = json!({ "pk": -2i32 });
        let r = call_it(&webview, body);

        // We filter using the account-pk, it doesn't check if the account exists. This is the reason
        // why it returns an empty vector instead of an error
        assert!(r.is_ok());
        assert_eq!(r.unwrap().len(), 0);
    }
}

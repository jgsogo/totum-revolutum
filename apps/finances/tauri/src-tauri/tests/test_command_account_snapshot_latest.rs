use finances_accounts::test_utils::establish_connection;
use finances_app_lib::models::Snapshot;
use serde_json::{json, Value};
use tauri::{test::MockRuntime, Manager, WebviewWindow};

fn call_it(webview: &WebviewWindow<MockRuntime>, body: Value) -> Result<Option<Snapshot>, Value> {
    tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: "account_snapshot_latest".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        },
    )
    .map(|b: tauri::ipc::InvokeResponseBody| b.deserialize::<Option<Snapshot>>().unwrap())
}

#[test]
fn test_account_snapshot_latest() {
    let pool = establish_connection();

    let app = finances_app_lib::create_app(tauri::test::mock_builder(), pool.clone());
    app.manage(pool); // FIXME: The `.manage` inside `create_app` is not working for the mock.
    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    {
        let body = json!({ "pk": 0i64 });
        let r = call_it(&webview, body);

        assert!(r.is_ok());
        let snapshot = r.unwrap();
        assert!(snapshot.is_some());
        let snapshot = snapshot.unwrap();
        assert_eq!(snapshot.account_id, 0i64);
    }

    {
        let body = json!({ "pk": -2i64 });
        let r = call_it(&webview, body);

        // We filter using the account-pk, it doesn't check if the account exists. This is the reason
        // why it returns an empty value instead of an error
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.is_none());
    }
}

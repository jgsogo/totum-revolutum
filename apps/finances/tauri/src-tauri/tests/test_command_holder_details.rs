use finances_accounts::test_utils::establish_connection;
use finances_app_lib::models::Holder;
use serde_json::{json, Value};
use tauri::{test::MockRuntime, Manager, WebviewWindow};

fn call_it(webview: &WebviewWindow<MockRuntime>, body: Value) -> Result<Holder, Value> {
    tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: "holder_details".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        },
    )
    .map(|b| b.deserialize::<Holder>().unwrap())
}

#[test]
fn test_holders() -> Result<(), Value> {
    let pool = establish_connection();

    let app = finances_app_lib::create_app(tauri::test::mock_builder(), pool.clone());
    app.manage(pool); // FIXME: The `.manage` inside `create_app` is not working for the mock.
    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    {
        let body = json!({"pk": 0i64});
        let r = call_it(&webview, body)?;
        assert_eq!(r.name, "holder0");
        assert!(!r.is_company);
    }

    Ok(())
}

use finances_accounts::test_utils::establish_connection;
use finances_app_lib::models::Account;
use serde_json::{json, Value};
use tauri::{test::MockRuntime, Manager, WebviewWindow};

fn call_it(webview: &WebviewWindow<MockRuntime>, body: Value) -> Result<Account, Value> {
    tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: "account_detail".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        },
    )
    .map(|b| b.deserialize::<Account>().unwrap())
}

#[test]
fn test_account_detail() -> Result<(), Value> {
    let pool = establish_connection();

    let app = finances_app_lib::create_app(tauri::test::mock_builder(), pool.clone());
    app.manage(pool); // FIXME: The `.manage` inside `create_app` is not working for the mock.
    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    {
        let body = json!({ "pk": 0i32 });
        let r = call_it(&webview, body)?;
        assert_eq!(r.name, "Gastos compartidos");
    }

    {
        let body = json!({ "pk": -2i32 });
        let r = call_it(&webview, body);

        assert!(r.is_err());
        let r = r.unwrap_err();
        assert_eq!(r.as_str().unwrap(), "Error loading account: Record not found");
    }
    Ok(())
}

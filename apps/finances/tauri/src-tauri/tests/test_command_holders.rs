use finances_accounts::test_utils::establish_connection;
use finances_app_lib::models::Holder;
use finances_app_lib::state::AppState;
use serde_json::{json, Value};
use tauri::{test::MockRuntime, Manager, WebviewWindow};

fn call_it(webview: &WebviewWindow<MockRuntime>, body: Value) -> Result<Vec<Holder>, Value> {
    tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: "holders".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        },
    )
    .map(|b| b.deserialize::<Vec<Holder>>().unwrap())
}

#[test]
fn test_holders() -> Result<(), Value> {
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
        let body = json!({});
        let r = call_it(&webview, body)?;
        assert_eq!(r.len(), 3);
        {
            let holder = r.get(0).unwrap();
            assert_eq!(holder.name, "holder0");
            assert!(!holder.is_company);
        }
        {
            let holder = r.get(1).unwrap();
            assert_eq!(holder.name, "holder1");
            assert!(!holder.is_company);
        }
        {
            let holder = r.get(2).unwrap();
            assert_eq!(holder.name, "holder2");
            assert!(holder.is_company);
        }
    }

    Ok(())
}

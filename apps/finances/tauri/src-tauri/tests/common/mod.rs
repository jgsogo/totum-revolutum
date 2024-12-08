use finances_accounts::test_utils::establish_connection;
use finances_app_lib::state::AppState;
use serde_json::Value;
use tauri::{test::MockRuntime, Manager, WebviewWindow};

pub fn webview() -> WebviewWindow<MockRuntime> {
    let pool = establish_connection();
    let app_state = AppState::new(
        "postgres_url".to_string(),
        "base_url".to_string(),
        "media_url".to_string(),
        "static_url".to_string(),
        "MKD".to_string(),
    );

    let app = finances_app_lib::create_app(tauri::test::mock_builder(), pool.clone(), app_state);
    app.manage(pool); // FIXME: The `.manage` inside `create_app` is not working for the mock.
    tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap()
}

pub fn call_it<T: for<'de> serde::de::Deserialize<'de>>(
    webview: &WebviewWindow<MockRuntime>,
    command: String,
    body: Value,
) -> Result<T, Value> {
    tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: command,
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        },
    )
    .map(|b| b.deserialize::<T>().unwrap())
}

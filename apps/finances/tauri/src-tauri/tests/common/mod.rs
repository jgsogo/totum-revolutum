use finances_accounts::test_utils::establish_connection;
use finances_app_models::AppState;
use serde_json::Value;
use tauri::{ipc::InvokeResponseBody, test::MockRuntime, Manager, WebviewWindow};

pub fn webview() -> WebviewWindow<MockRuntime> {
    let pool = establish_connection();
    let app_state = AppState::new(
        "postgres_url".to_string(),
        "base_url".to_string(),
        "media_url".to_string(),
        "static_url".to_string(),
        "USD".to_string(),
    );

    let app = finances_app_lib::create_app(tauri::test::mock_builder(), pool.clone(), app_state.clone());
    // FIXME: The `.manage` inside `create_app` is not working for the mock.
    app.manage(app_state);

    let mut conn = pool.get().expect("Get a connection from the Pool");
    let main_context = finances_app_lib::get_main_context(&mut conn).expect("Error creating main context");
    app.manage(main_context);

    app.manage(pool);
    tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap()
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn call_it_proto<T: TryFrom<Vec<u8>>>(
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
    .map(|b| match b {
        InvokeResponseBody::Raw(v) => T::try_from(v)
            .map_err(|_| "Cannot parse from Vec<u8>".to_string())
            .unwrap(),
        _ => panic!("JSON response. Not expected"),
    })
}

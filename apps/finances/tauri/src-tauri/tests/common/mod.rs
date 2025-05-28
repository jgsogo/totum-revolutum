use finances_accounts::test_utils::establish_connection;
use finances_app_models::{google_type, AppState, DatabaseConnection};

use serde_json::Value;
use tauri::{test::MockRuntime, Manager, WebviewWindow};

pub fn webview() -> WebviewWindow<MockRuntime> {
    let pool = establish_connection();
    let db = DatabaseConnection::new("user", "password", "host", 1234, "dbname");
    let app_state = AppState::new(
        google_type::CurrencyCode::USD,
        "media_url".to_string(),
        "static_url".to_string(),
        "base_url".to_string(),
        db,
        &camino::Utf8PathBuf::from("backup_dir"),
        None,
    );

    let app = finances_app_lib::create_app(tauri::test::mock_builder(), pool.clone(), app_state.clone(), 0, None);
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

pub struct InvokeBody(tauri::ipc::InvokeBody);

impl From<Value> for InvokeBody {
    fn from(value: Value) -> Self {
        Self(tauri::ipc::InvokeBody::Json(value))
    }
}

impl From<Vec<u8>> for InvokeBody {
    fn from(value: Vec<u8>) -> Self {
        Self(tauri::ipc::InvokeBody::Raw(value))
    }
}

#[derive(Debug)]
pub struct InvokeResponseBody(tauri::ipc::InvokeResponseBody);

impl InvokeResponseBody {
    #[allow(dead_code)]
    pub fn try_into_json<T: for<'de> serde::de::Deserialize<'de>>(self) -> Result<T, serde_json::Error> {
        self.0.deserialize::<T>()
    }

    pub fn try_into_proto<T: TryFrom<Vec<u8>>>(self) -> Result<T, String> {
        match self.0 {
            tauri::ipc::InvokeResponseBody::Raw(v) => {
                T::try_from(v).map_err(|_| "Cannot parse from Vec<u8>".to_string())
            }
            tauri::ipc::InvokeResponseBody::Json(_) => Err("JSON response. Not expected".to_string()),
        }
    }
}

pub fn call_command(
    webview: &WebviewWindow<MockRuntime>,
    command: &str,
    body: InvokeBody,
) -> Result<InvokeResponseBody, Value> {
    tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: command.to_owned(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body: body.0,
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        },
    )
    .map(InvokeResponseBody)
}

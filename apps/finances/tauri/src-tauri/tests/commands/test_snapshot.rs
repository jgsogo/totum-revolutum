use finances_accounts::test_utils::establish_connection;
use finances_app_lib::models::Snapshot;
use finances_app_lib::state::AppState;
use serde_json::{json, Value};
use tauri::{test::MockRuntime, Manager, WebviewWindow};

fn call_it<T: for<'de> serde::de::Deserialize<'de>>(
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

#[test]
fn test_snapshot() {
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

    /****
    Non-numerable account
    ***/
    let account_id = 1i32;
    {
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 0);
    }

    // Snapshot (non-numerable)
    {
        let body = json!({
            "accountPk": account_id,
            "dateValue": "2024-11-30",
            "amount": Some(100f32),
        });

        let r = call_it::<usize>(&webview, "create_snapshot".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());

        // Now we have one more snapshot
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 1);
    }

    /****
    Numerable account
    ***/
    let account_id = 8i32;
    {
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 0);
    }

    // Snapshot (non-numerable)
    {
        let body = json!({
            "accountPk": account_id,
            "dateValue": "2024-11-30",
            "amount": Some(300f32),
            "quantity": Some(3f32),
            "unitValue": Some(100f32),

        });

        let r = call_it::<usize>(&webview, "create_snapshot".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());

        // Now we have one more snapshot
        let body = json!({ "pk": account_id });
        let r = call_it::<Vec<Snapshot>>(&webview, "account_snapshots".to_string(), body);
        assert!(r.is_ok(), "Error: {}", r.unwrap_err());
        assert_eq!(r.unwrap().len(), 1);
    }
}

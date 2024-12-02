use finances_accounts::test_utils::establish_connection;
use finances_app_lib::models::Account;
use finances_app_lib::state::AppState;
use serde_json::{json, Value};
use tauri::{test::MockRuntime, Manager, WebviewWindow};

fn call_it(webview: &WebviewWindow<MockRuntime>, command: String, body: Value) -> Result<Vec<Account>, Value> {
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
    .map(|b| b.deserialize::<Vec<Account>>().unwrap())
}

#[test]
fn test_acount_lists() {
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

    // All accounts
    {
        let body = json!({ "holderPk": 0i64 });
        let r = call_it(&webview, "all_accounts".to_string(), body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 7);

        let account_names = r.into_iter().map(|acc| acc.name).collect::<Vec<String>>();
        assert_eq!(
            account_names,
            vec![
                "Gastos compartidos",
                "Depósito 3M",
                "Hipoteca casa NY",
                "IBM",
                "Indexa Capital",
                "Plan de pensiones",
                "IBM2",
            ]
        );
    }

    // Savings accounts
    {
        let body = json!({ "holderPk": 0i64 });
        let r = call_it(&webview, "savings_accounts".to_string(), body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 2);

        let account_names = r.into_iter().map(|acc| acc.name).collect::<Vec<String>>();
        assert_eq!(account_names, vec!["Gastos compartidos", "Depósito 3M"]);
    }

    // Investments accounts
    {
        let body = json!({ "holderPk": 0i64 });
        let r = call_it(&webview, "investment_accounts".to_string(), body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 3);

        let account_names = r.into_iter().map(|acc| acc.name).collect::<Vec<String>>();
        assert_eq!(account_names, vec!["IBM", "Indexa Capital", "IBM2"]);
    }

    // Retirement accounts
    {
        let body = json!({ "holderPk": 0i64 });
        let r = call_it(&webview, "retirement_accounts".to_string(), body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 1);

        let account_names = r.into_iter().map(|acc| acc.name).collect::<Vec<String>>();
        assert_eq!(account_names, vec!["Plan de pensiones"]);
    }
}

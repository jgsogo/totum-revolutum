use finances_app_lib::models::MenuGroup;
use finances_db::test_utils::fixtures::database_with_accounts;
use serde_json::{json, Value};
use tauri::{test::MockRuntime, Manager, WebviewWindow};

fn call_it(webview: &WebviewWindow<MockRuntime>, body: Value) -> Result<Vec<MenuGroup>, Value> {
    tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: "sidebar_menu".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::Json(body),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        },
    )
    .map(|b| b.deserialize::<Vec<finances_app_lib::models::MenuGroup>>().unwrap())
}

#[test]
fn test_category_all() {
    let database = database_with_accounts();
    let pool = finances_app_lib::db::establish_connection(database.filepath().to_str().unwrap());

    let app = finances_app_lib::create_app(tauri::test::mock_builder(), pool.clone());
    app.manage(pool); // FIXME: The `.manage` inside `create_app` is not working for the mock.
    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    {
        let body = json!({ "category": "/all" });
        let r = call_it(&webview, body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 2);

        {
            let group = r.get(0).unwrap();
            assert_eq!(group.name, "holder0");
            assert_eq!(group.accounts.len(), 1);
            assert_eq!(group.accounts.get(0).unwrap().name, "Gastos compartidos");
        }
        {
            let group = r.get(1).unwrap();
            assert_eq!(group.name, "holder1");
            assert_eq!(group.accounts.len(), 1);
            assert_eq!(group.accounts.get(0).unwrap().name, "IBM");
        }
    }

    {
        let body = json!({ "category": "/accounts" });
        let r = call_it(&webview, body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 1);

        {
            let group = r.get(0).unwrap();
            assert_eq!(group.name, "holder0");
            assert_eq!(group.accounts.len(), 1);
            assert_eq!(group.accounts.get(0).unwrap().name, "Gastos compartidos");
        }
    }

    {
        let body = json!({ "category": "/investments" });
        let r = call_it(&webview, body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 1);

        {
            let group = r.get(0).unwrap();
            assert_eq!(group.name, "holder1");
            assert_eq!(group.accounts.len(), 1);
            assert_eq!(group.accounts.get(0).unwrap().name, "IBM");
        }
    }

    {
        let body = json!({ "category": "/retirement" });
        let r = call_it(&webview, body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 0);
    }

    {
        let body = json!({ "category": "/rentals" });
        let r = call_it(&webview, body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 0);
    }

    {
        let body = json!({ "category": "/taxes" });
        let r = call_it(&webview, body);

        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(r.len(), 0);
    }

    // An invalid category
    {
        let body = json!({ "category": "<not-valid>" });
        let r = call_it(&webview, body);

        assert!(r.is_err());
        let r = r.unwrap_err();
        assert_eq!(r.as_str().unwrap(), "Unexpected sidebar_menu category '<not-valid>'");
    }
}

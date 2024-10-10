use finances_db::test_utils::fixtures::database_with_accounts;

#[test]
fn test_category_all() {
    let database = database_with_accounts();
    let pool = finances_app_lib::db::establish_connection(database.filepath().to_str().unwrap());

    // TODO: The the actual sidebar_menu command. But to do it, first I need to create a mocked database,
    // and to create the database I need the migrations...

    let app = finances_app_lib::create_app(tauri::test::mock_builder(), pool);
    let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    let r = tauri::test::get_ipc_response(
        &webview,
        tauri::webview::InvokeRequest {
            cmd: "ping".into(),
            callback: tauri::ipc::CallbackFn(0),
            error: tauri::ipc::CallbackFn(1),
            url: "http://tauri.localhost".parse().unwrap(),
            body: tauri::ipc::InvokeBody::default(),
            headers: Default::default(),
            invoke_key: tauri::test::INVOKE_KEY.to_string(),
        },
    )
    .map(|b| b.deserialize::<String>().unwrap());

    assert!(r.is_err());
    assert_eq!(r.unwrap_err().as_str(), Some("Command ping not found"));
}

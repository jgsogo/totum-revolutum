use tauri::Manager;
mod db;
mod menu;
pub mod views;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // TODO: See mutability example in the App::manage method. It shows how to update the connection. Of course we don't want here a hardcoded string
    let database_url = "postgres://finances_ro:finances_ro@localhost/finances";
    let pool = db::establish_connection(database_url);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout))
                .build(),
        )
        .setup(|app| {
            app.manage(pool);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            menu::sidebar_menu,
            views::account_detail::detail_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

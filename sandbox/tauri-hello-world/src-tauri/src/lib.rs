use log::info;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    info!("tauri::command - greet");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn datatables_table() -> Vec<(i32, String, i32, String)> {
    info!("tauri::command - datatables_table");
    vec![
        (0, "name0".into(), 10, "city0".into()),
        (1, "name1".into(), 11, "city1".into()),
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, datatables_table])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use tauri::Manager;
mod db;
use tauri::State;

#[derive(serde::Serialize)]
struct Account {
    pub holder: String,
    pub name: String,
    pub r#type: String,
    pub ccy: String,

    pub href: String,
    pub labels: Vec<String>,
}

#[tauri::command]
async fn accounts(_pool: State<'_, Pool<ConnectionManager<PgConnection>>>) -> Result<Vec<Account>, String> {
    let ten_millis = std::time::Duration::from_millis(2000);
    std::thread::sleep(ten_millis);

    let accounts = vec![
        Account {
            holder: "holder".into(),
            name: "account1".into(),
            r#type: "type1".into(),
            ccy: "EUR".into(),
            href: "/accounts/pk/1".into(),
            labels: vec!["label1".into(), "label2".into()],
        },
        Account {
            holder: "holder".into(),
            name: "account2".into(),
            r#type: "type2".into(),
            ccy: "EUR".into(),
            href: "/accounts/pk/2".into(),
            labels: vec!["label1".into(), "label3".into()],
        },
    ];
    Ok(accounts)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // TODO: See mutability example in the App::manage method. It shows how to update the connection. Of course we don't want here a hardcoded string
    let database_url = "postgres://finances_ro:finances_ro@localhost/finances";
    let pool = db::establish_connection(database_url);

    tauri::Builder::default()
        .setup(|app| {
            app.manage(pool);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![accounts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

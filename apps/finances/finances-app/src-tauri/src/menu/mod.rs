use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use tauri::State;

#[derive(serde::Serialize)]
pub struct Account {
    pub holder: String,
    pub name: String,
    pub r#type: String,
    pub ccy: String,

    pub href: String,
    pub labels: Vec<String>,
}

#[tauri::command]
pub async fn accounts(
    _pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
    category: &str,
) -> Result<Vec<Account>, String> {
    log::info!("Get Accounts for category {category}");

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

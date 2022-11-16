use pcloud_sdk_desktop::storage;

/// Show stats contained within the project folder
pub fn _project_details(config: &storage::config::Config) {
    println!("client_id: {}", config.auth.client_id);

    // TODO: Translate client_id to application name
    println!("userid: {}", config.auth.userid);
    println!("action: {:#?}", config.action.action);
}

/// Show data about last execution and next scheduled one
pub fn _cron_details(config: &storage::config::Config, directory: &storage::cron::Directory) {
    let last_executed = match config.action.last_executed {
        Some(d) => d.to_string(),
        None => "NEVER".into(),
    };
    println!("   last: {}", last_executed);

    if let Some(upcoming) = directory.upcoming() {
        println!("   next: {}", upcoming);
    }
}

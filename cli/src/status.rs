use clap::Args;

use path_clean::PathClean;
use pcloud_sdk_desktop::storage;
use pcloud_sdk_desktop::LockedFileTrait;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use tracing::debug;

#[derive(Args, Debug)]
pub struct StatusParams {
    /// Where to run this command, if directory doesn't exist, it will be created
    directory: Option<PathBuf>,
}

// fn format_offsetdatetime(
//     timestamp: &chrono::DateTime<chrono::Utc>,
// ) -> Result<String, time::error::Format> {
//     let format = time::format_description::parse(
//         "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
//              sign:mandatory]:[offset_minute]:[offset_second]",
//     )
//     .unwrap();

//     // let local = chrono::DateTime::from(timestamp);
//     // TODO: Convert to local offset
//     timestamp.format(&format)
// }

pub fn handle(_home: &Path, params: &StatusParams) {
    let working_dir = (match &params.directory {
        Some(d) => {
            if d.is_absolute() {
                d.to_path_buf()
            } else {
                env::current_dir()
                    .expect("Cannot return current dir")
                    .join(d)
            }
        }
        None => env::current_dir().expect("Cannot return current directory"),
    })
    .clean();
    debug!("Status pCloud folder {}", working_dir.display());

    // TODO: Check if folder is already a pCloud folder

    let config_data = storage::config::ConfigData::read(&working_dir);
    let config = config_data.config();
    println!("client_id: {}", config.auth.client_id);
    println!("userid: {}", config.auth.userid);
    // TODO: println!("action: {}", config.action.action);
    println!(
        "last_executed: {}",
        config
            .action
            .last_executed
            .map_or("NEVER".to_string(), |l| {
                chrono::DateTime::<chrono::Local>::from(l).to_string()
            })
    );
    println!(
        "next_execution: {}",
        config
            .action
            .upcoming()
            .map_or("NOT SCHEDULED".to_string(), |l| {
                l.with_timezone(&chrono::Local).to_string()
            })
    );
    // TODO: Show stats about files in this folder
}

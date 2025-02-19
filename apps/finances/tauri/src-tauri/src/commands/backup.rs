use crate::{Error, Result};
use finances_app_models::AppState;
use postgresql_commands::pg_dump::PgDumpBuilder;
use postgresql_commands::traits::CommandToString;
use postgresql_commands::CommandBuilder;
use postgresql_commands::CommandExecutor;
use tauri::State;
use tempfile::TempDir;

#[tauri::command]
pub async fn do_backup(app_state: State<'_, AppState>) -> Result<String> {
    // Do a backup of all the Tauri application data:
    let tmp_dir = TempDir::new().map_err(|e| Error::Other(format!("Failed to create temp folder: {e}")))?;

    // Postgresql database
    let db = app_state.db()?;
    let pg_dump_file = tmp_dir.path().join("database.dump");

    let mut pg_dump = PgDumpBuilder::new()
        .host(db.host())
        .port(db.port())
        .username(db.user())
        .pg_password(db.password())
        .dbname(db.dbname())
        .file(&pg_dump_file)
        .build();

    let command = pg_dump.to_command_string();
    let (stdout, stderr) = pg_dump
        .execute()
        .map_err(|e| Error::Other(format!("Error executing the command: {e}")))?;

    let command_output = format!("COMMAND:\n{command}\n\nSTDOUT:\n{stdout}\n\nSTDERR:\n{stderr}");

    // Media files

    Ok(command_output)
}

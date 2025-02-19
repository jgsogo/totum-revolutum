use crate::types::ConnectionType;
use crate::PgConnection;
use crate::{Error, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{AccountHolder, Custodian};
use finances_app_models::AppState;
use finances_app_models::DatabaseConnection;
use postgresql_commands::pg_dump::PgDumpBuilder;
use postgresql_commands::traits::CommandToString;
use postgresql_commands::CommandBuilder;
use postgresql_commands::CommandExecutor;
use tauri::State;
use tempfile::TempDir;

#[tauri::command]
pub async fn do_backup(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    app_state: State<'_, AppState>,
) -> Result<String> {
    log::info!("Do backup!");

    // Do a backup of all the Tauri application data:
    // let tmp_dir = TempDir::new().map_err(|e| Error::Other(format!("Failed to create temp folder: {e}")))?;
    // let tmp_dir_path = tmp_dir.path();
    let tmp_dir = std::path::PathBuf::from("/Users/jgsogo/personal/totum-revolutum/t1");
    let tmp_dir_path = &tmp_dir;

    // Postgresql database
    let pg_dump_file = tmp_dir_path.join("database.dump");
    let command_output = backup_database(app_state.db()?, &pg_dump_file)?;
    log::info!(" - DB backup created at {:?}", pg_dump_file);

    // Media files
    let mut conn = pool.get().expect("Get a connection from the Pool");
    let base_media_url = format!("{}{}", app_state.base_url()?, app_state.base_media_url()?); // FIXME: AppState should take care of this
    media_files(&base_media_url, &mut conn, tmp_dir_path).await?;

    Ok(command_output)
}

fn backup_database(db: &DatabaseConnection, dump_file: &std::path::Path) -> Result<String> {
    let mut pg_dump = PgDumpBuilder::new()
        .host(db.host())
        .port(db.port())
        .username(db.user())
        .pg_password(db.password())
        .dbname(db.dbname())
        .file(dump_file)
        .build();

    let command = pg_dump.to_command_string();
    let (stdout, stderr) = pg_dump
        .execute()
        .map_err(|e| Error::Other(format!("Error executing the command: {e}")))?;

    let command_output = format!("COMMAND:\n{command}\n\nSTDOUT:\n{stdout}\n\nSTDERR:\n{stderr}");

    Ok(command_output)
}

async fn media_files(base_media_url: &str, conn: &mut PgConnection, dump_directory: &std::path::Path) -> Result<()> {
    // Collect media from custodians
    for custodian in Custodian::all()
        .select(Custodian::as_select())
        .load::<Custodian>(conn)?
    {
        if let Some(photo) = custodian.photo {
            if !photo.is_empty() {
                let url = format!("{}{}", base_media_url, photo); // FIXME: AppState should take care of this
                log::info!(" - Backup custodian photo: {:?}", url);
                let out = backup_url_file(&url, dump_directory).await?;
                log::info!(" - Backup custodian photo: {:?} into {:?}", url, out);
            }
        }
    }

    // Collect media from holders
    for holder in AccountHolder::all()
        .select(AccountHolder::as_select())
        .load::<AccountHolder>(conn)?
    {
        if let Some(photo) = holder.photo {
            if !photo.is_empty() {
                let url = format!("{}{}", base_media_url, photo); // FIXME: AppState should take care of this
                log::info!(" - Backup holder photo: {:?}", url);
                let out = backup_url_file(&url, dump_directory).await?;
                log::info!(" - Backup custodian photo: {:?} into {:?}", url, out);
            }
        }
    }
    Ok(())
}

async fn backup_url_file(url: &str, output_dir: &std::path::Path) -> Result<std::path::PathBuf> {
    let resp = reqwest::get(url)
        .await
        .map_err(|e| Error::Other(format!("Error making the request: {e}")))?;
    let body = resp
        .text()
        .await
        .map_err(|e| Error::Other(format!("Error getting the body from the request: {e}")))?;
    let output_filename = url
        .rsplit('/')
        .collect::<Vec<_>>()
        .first()
        .map(|v| v.to_string())
        .unwrap();
    let output_path = output_dir.join(output_filename);
    let mut out =
        std::fs::File::create(&output_path).map_err(|e| Error::Other(format!("Error creating the local file: {e}")))?;
    std::io::copy(&mut body.as_bytes(), &mut out)
        .map_err(|e| Error::Other(format!("Error copying the request content into the file: {e}")))?;
    Ok(output_path)
}

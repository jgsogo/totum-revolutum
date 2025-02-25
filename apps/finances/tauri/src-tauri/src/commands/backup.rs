use std::io::Write;

use crate::types::ConnectionType;
use crate::PgConnection;
use crate::{Error, Result};
use camino::Utf8Path;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{AccountHolder, Custodian};
use finances_app_models::AppState;
use finances_app_models::DatabaseConnection;
use futures_util::StreamExt;
use postgresql_commands::pg_dump::PgDumpBuilder;
use postgresql_commands::traits::CommandToString;
use postgresql_commands::CommandBuilder;
use postgresql_commands::CommandExecutor;
use tauri::State;

#[tauri::command]
pub async fn do_backup(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    app_state: State<'_, AppState>,
) -> Result<String> {
    log::info!("Do backup!");

    // Do a backup of all the Tauri application data:
    let tmp_dir_path = app_state.backup_directory();
    let today = chrono::Utc::now().naive_utc().format("%Y-%m-%d").to_string();
    let tmp_dir_path = tmp_dir_path.join(&today);
    std::fs::create_dir_all(&tmp_dir_path)
        .map_err(|e| Error::Other(format!("Cannot create folder for backup: {e}")))?;

    // Postgresql database
    let pg_dump_file = tmp_dir_path.join("database.dump");
    let command_output = backup_database(app_state.db()?, &pg_dump_file)?;
    log::info!(" - DB backup created at {:?}", pg_dump_file);

    // Media files
    let mut conn = pool.get().expect("Get a connection from the Pool");
    let base_media_url = format!("{}{}", app_state.base_url()?, app_state.base_media_url()?); // FIXME: AppState should take care of this
    let tmp_dir_media = tmp_dir_path.join("media");
    media_files(&base_media_url, &mut conn, &tmp_dir_media).await?;

    Ok(command_output)
}

fn backup_database(db: &DatabaseConnection, dump_file: &Utf8Path) -> Result<String> {
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

async fn media_files(base_media_url: &str, conn: &mut PgConnection, dump_directory: &Utf8Path) -> Result<()> {
    // Collect media from custodians
    for custodian in Custodian::all()
        .select(Custodian::as_select())
        .load::<Custodian>(conn)?
    {
        if let Some(photo) = custodian.photo {
            if !photo.is_empty() {
                let url = format!("{}{}", base_media_url, photo); // FIXME: AppState should take care of this
                log::info!(" - Backup custodian photo: {:?}", url);
                let output_path = dump_directory.join(photo);
                backup_url_file(&url, &output_path).await?;
                log::info!(" - Backup custodian photo: {:?} into {:?}", url, output_path);
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
                let output_path = dump_directory.join(photo);
                backup_url_file(&url, &output_path).await?;
                log::info!(" - Backup custodian photo: {:?} into {:?}", url, output_path);
            }
        }
    }
    Ok(())
}

async fn backup_url_file(url: &str, output_path: &Utf8Path) -> Result<()> {
    let mut out = {
        let prefix = output_path
            .parent()
            .ok_or(Error::Other("No parent folder".to_string()))?;
        std::fs::create_dir_all(prefix).map_err(|e| Error::Other(format!("Cannot create folder for file: {e}")))?;
        std::fs::File::create(output_path).map_err(|e| Error::Other(format!("Error creating the local file: {e}")))?
    };

    let mut stream = reqwest::get(url)
        .await
        .map_err(|e| Error::Other(format!("Error making the request: {e}")))?
        .bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| Error::Other(format!("Error getting next chunk: {e}")))?;
        out.write_all(&chunk)
            .map_err(|e| Error::Other(format!("Error writting chunk to file: {e}")))?;
    }

    out.flush()
        .map_err(|e| Error::Other(format!("Error flushing file: {e}")))?;

    Ok(())
}

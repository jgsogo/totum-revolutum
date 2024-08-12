use diesel::*;

use diesel_utils::managers::GetByPkManager;
use pcloud_sdk::types::FileID;

use crate::database::models;
use crate::database::models::Formats;
use crate::database::pcloud_database::run_migrations;

type TestConnection = SqliteConnection;

#[test]
fn test_models_photo_file() {
    let mut conn = connection();

    sql_query("INSERT INTO files (id, name, directory_id, hash, size) VALUES (1, 'file.png', 0, 'hash', 100)")
        .execute(&mut conn)
        .unwrap();

    // PhotoFile with empty metadata
    let file_ = models::File::get_by_pk(1, &mut conn).unwrap();
    let fileid_ = FileID::new(1);
    let format_ = models::Format::find(&Formats::Png, &mut conn).unwrap();
    let json_metadata = serde_json::json!({});
    let new_photo_file = models::PhotoFile::new_from(&file_, &fileid_, &format_, true, json_metadata);

    use crate::database::schema::photo_files::dsl::*;
    let photo = diesel::insert_into(photo_files)
        .values(&new_photo_file)
        .returning(models::PhotoFile::as_returning())
        .get_result(&mut conn)
        .unwrap();

    assert_eq!(photo.file_id, 1);
    assert_eq!(photo.fileid, fileid_.inner() as i64);
    assert_eq!(photo.format_id, format_.id);
    assert!(photo.processed);
    assert!(photo.metadata.as_json_value().is_object());
}

#[test]
fn test_models_photo_file_with_metadata() {
    let mut conn = connection();

    sql_query("INSERT INTO files (id, name, directory_id, hash, size) VALUES (1, 'file.png', 0, 'hash', 100)")
        .execute(&mut conn)
        .unwrap();

    // PhotoFile with empty metadata
    let file_ = models::File::get_by_pk(1, &mut conn).unwrap();
    let fileid_ = FileID::new(1);
    let format_ = models::Format::find(&Formats::Png, &mut conn).unwrap();
    let mut json_metadata = serde_json::json!({});
    json_metadata["value_int"] = serde_json::json!(42);
    json_metadata["value_str"] = serde_json::json!("string");
    json_metadata["value_list"] = serde_json::json!([1, 2, 3]);
    json_metadata["value_dict"] = serde_json::json!({"a": 1, "b": "b-str"});
    let new_photo_file = models::PhotoFile::new_from(&file_, &fileid_, &format_, true, json_metadata);

    use crate::database::schema::photo_files::dsl::*;
    let photo = diesel::insert_into(photo_files)
        .values(&new_photo_file)
        .returning(models::PhotoFile::as_returning())
        .get_result(&mut conn)
        .unwrap();

    let metadata_ = photo.metadata.as_json_value();
    assert_eq!(metadata_["value_int"], 42);
    assert_eq!(metadata_["value_str"], "string");
    assert_eq!(metadata_["value_list"].as_array().unwrap(), &vec![1, 2, 3]);
    let map = metadata_["value_dict"].as_object().unwrap();
    assert_eq!(map.get("a").unwrap(), 1);
    assert_eq!(map.get("b").unwrap(), "b-str");
}

fn connection() -> TestConnection {
    let mut conn = SqliteConnection::establish(":memory:").unwrap();
    diesel::sql_query("PRAGMA foreign_keys = ON") // Enables foreign keys support: https://www.sqlite.org/foreignkeys.html
        .execute(&mut conn)
        .unwrap();

    run_migrations(&mut conn).unwrap();

    conn.begin_test_transaction().unwrap();
    conn
}

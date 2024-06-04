use diesel::prelude::*;

// TODO: Deserialize as FileID
// TODO: Consider using the FILEID as the unique id

#[derive(Insertable)]
#[diesel(table_name = crate::schema::photos)]
pub struct NewPhoto<'a> {
    pub fileid: &'a i64,
    pub path: &'a str,
    pub processed: &'a bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::photos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Photo {
    /// Autoincrement ID as database index
    pub id: i32,

    /// The [`pcloud_sdk::types::FileID`] of this file in the PCloud storage
    pub fileid: i64,

    /// The relative path of this file in the PCloud storage
    pub path: String,

    /// Indicates whether this file has already been processed or not.
    ///
    /// This value is usually true when the files have been added using this application, because
    /// it has run all the processing before uploading them (resize, conversion, metadata,...);
    /// however, when the files are discovered in the remote storage we initialize this flag
    /// as False and some future work need to be done on this file.
    ///
    /// This flag will also be useful if we added some new processing and we want to work on
    /// all the files again. We just need to update this flag to `false` for all the table.
    pub processed: bool,
}

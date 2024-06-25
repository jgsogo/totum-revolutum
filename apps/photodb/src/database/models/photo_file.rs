use diesel::prelude::*;

use pcloud_sdk::types::FileID;

use super::{File, Format};

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::photo_files)]
pub struct NewPhotoFile {
    file_id: i32,
    fileid: i64,
    format_id: i32,
    processed: bool,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::database::schema::photo_files)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PhotoFile {
    /// Primary key, but also FK to [`File`]
    pub file_id: i32,

    /// The [`FileID`] of this file in the PCloud storage
    /// TODO: Serialize/deserialized as a FileID object
    pub fileid: i64,

    /// A FK to [`super::Format`]
    pub format_id: i32,

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

impl PhotoFile {
    pub fn new_from(file: &File, fileid: &FileID, format: &Format, processed: bool) -> NewPhotoFile {
        NewPhotoFile {
            file_id: file.id,
            fileid: fileid.inner() as i64,
            format_id: format.id,
            processed,
        }
    }
}

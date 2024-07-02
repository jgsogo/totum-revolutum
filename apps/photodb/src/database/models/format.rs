use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;

use anyhow::Result;
use diesel::associations::HasTable;
use diesel::connection::LoadConnection;
use diesel::dsl::Eq;
use diesel::query_dsl::methods::FilterDsl;
use diesel::query_dsl::LoadQuery;
use diesel::*;
use lazy_static::lazy_static;
use log::warn;
use strum_macros::{Display, EnumString};

use super::super::schema::*;

lazy_static! {
    static ref FORMATS_CACHE: Mutex<HashMap<String, Format>> = Mutex::new(HashMap::new());
}

#[derive(PartialEq, Eq, Display, Debug, Clone, EnumString)]
#[allow(clippy::upper_case_acronyms)]
#[strum(serialize_all = "snake_case")]
pub enum Formats {
    Unknown,
    PNG,
}

impl Formats {
    /// Returns the typical file extension for the given format. [`Formats::Unknown`] returns `None`.
    pub fn as_extension(&self) -> Option<String> {
        match self {
            Formats::Unknown => None,
            v => Some(v.to_string()),
        }
    }
}

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName, Selectable,
)]
#[diesel(table_name = formats)]
pub struct Format {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub format: String,
}

#[derive(Insertable)]
#[diesel(table_name = formats)]
pub struct NewFormat<'a> {
    pub parent_id: Option<i32>,
    pub format: &'a str,
}

impl Format {
    /// Looks for the [`Format`] entry for the given `value`.
    ///
    /// This method uses LRU cache, so it won't hit the database for the already queried values.
    pub fn find<Conn: LoadConnection>(value: Formats, conn: &mut Conn) -> Result<Format>
    where
        for<'a> <<Self as HasTable>::Table as FilterDsl<Eq<formats::format, &'a str>>>::Output:
            RunQueryDsl<Conn> + LoadQuery<'a, Conn, Self>,
    {
        let str = value.to_string();

        let mut cache_ = FORMATS_CACHE.lock().unwrap();
        let values = match cache_.entry(value.to_string()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                use crate::database::schema::formats::dsl::*;
                let format_: Format = FilterDsl::filter(formats, format.eq(str.as_str())).get_result(conn)?;
                v.insert(format_)
            }
        };

        Ok(values.clone())
    }

    pub fn format(&self) -> Formats {
        match Formats::from_str(&self.format) {
            Ok(f) => f,
            Err(_) => {
                warn!("Format value '{}' not found in Formats enum", self.format);
                Formats::Unknown
            }
        }
    }
}

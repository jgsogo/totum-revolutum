use std::io;
use std::path::Path;

use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use exif::{Exif, In, Tag};

/// Returns the [`Exif`] data from a file path
pub fn get_exif_data<P: AsRef<Path>>(path: P) -> Result<Exif> {
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    get_exif_data_from_memory(&mut bufreader)
}

/// Returns the [`Exif`] data from a buffer
pub fn get_exif_data_from_memory<R: io::BufRead + io::Seek>(bufreader: &mut R) -> Result<Exif> {
    let exifreader = exif::Reader::new();
    Ok(exifreader.read_from_container(bufreader)?)
}

/// Returns the creation data from the EXIF data. It will try the following EXIF tags in order:
/// [`Tag::DateTimeOriginal`], [`Tag::DateTime`] and [`Tag::DateTimeDigitized`].
pub fn get_creation_date(exif: &Exif) -> Result<NaiveDateTime> {
    let date = if let Some(date) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        Ok(date)
    } else if let Some(date) = exif.get_field(Tag::DateTime, In::PRIMARY) {
        Ok(date)
    } else if let Some(date) = exif.get_field(Tag::DateTimeDigitized, In::PRIMARY) {
        Ok(date)
    } else {
        Err(anyhow!("Cannot find any Date in EXIF data"))
    };

    match date {
        Ok(date) => {
            let value = date.display_value().with_unit(exif).to_string();
            let no_timezone = NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S")?;
            Ok(no_timezone)
        }
        Err(e) => Err(e),
    }
}

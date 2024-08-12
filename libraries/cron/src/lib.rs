use chrono::{DateTime, TimeZone};
use cron_parser::parse;
use serde::{Deserialize, Serialize};

pub use error::{Error, Result};

pub mod error;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CronTz {
    pub expression: String,
    tz: String,
}

impl CronTz {
    pub fn new(expression: &str, tz: &chrono_tz::Tz) -> Result<Self> {
        // Validate
        let now_tz = tz.from_utc_datetime(&chrono::Utc::now().naive_utc());
        parse(expression, &now_tz)?;

        // Create the object
        Ok(Self {
            expression: expression.to_string(),
            tz: tz.to_string(),
        })
    }

    pub fn cron_tz(&self) -> Result<chrono_tz::Tz> {
        self.tz
            .parse()
            .map_err(|e: chrono_tz::ParseError| Error::Other(e.to_string()))
    }

    pub fn upcoming(&self) -> Result<DateTime<chrono_tz::Tz>> {
        let tz = self.cron_tz()?;
        let now_tz = tz.from_utc_datetime(&chrono::Utc::now().naive_utc());
        self.next(&now_tz)
    }

    pub fn next<Tz>(&self, previous: &DateTime<Tz>) -> Result<DateTime<Tz>>
    where
        Tz: TimeZone,
    {
        Ok(parse(&self.expression, previous)?)
    }
}

use chrono::{DateTime, TimeZone};
use cron_parser::parse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CronTz {
    pub expression: String,
    tz: String,
}

impl CronTz {
    pub fn new(expression: &str, tz: &chrono_tz::Tz) -> Self {
        // Validate
        let now_tz = tz
            .from_local_datetime(&chrono::Utc::now().naive_utc())
            .unwrap();
        parse(expression, &now_tz).unwrap();

        // Create the object
        Self {
            expression: expression.to_string(),
            tz: tz.to_string(),
        }
    }

    pub fn cron_tz(&self) -> chrono_tz::Tz {
        self.tz.parse().unwrap()
    }

    pub fn upcoming(&self) -> Option<DateTime<chrono_tz::Tz>> {
        let tz = self.cron_tz();

        let now_tz = tz
            .from_local_datetime(&chrono::Utc::now().naive_utc())
            .unwrap();

        let schedule = parse(&self.expression, &now_tz).unwrap();
        Some(schedule)
    }
}

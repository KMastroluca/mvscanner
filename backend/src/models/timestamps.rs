use std::fmt::{Display, Formatter};

use serde::{de::Error, Deserialize, Serialize};

use chrono::NaiveDate;
use serde::Deserializer;

use super::residents::Resident;

#[derive(Debug, Deserialize)]
pub struct RangeParams {
    #[serde(deserialize_with = "deserialize_date")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    pub end_date: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResidentTimestamp {
    pub resident: Resident,
    pub timestamp: TimeStamp,
}

// Deserialize date strings into NaiveDate
fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(serde::de::Error::custom)
}

impl Display for PostTimestamp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "RFID: {}, Location: {}", self.rfid, self.location)
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, Eq, PartialEq)]
pub struct PostTimestamp {
    pub rfid: String,
    pub location: usize,
}

impl From<PostTimestamp> for TimeStamp {
    fn from(ts: PostTimestamp) -> Self {
        Self {
            rfid: ts.rfid,
            location: ts.location,
            time: None,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, Eq, PartialEq)]
pub struct TimeStamp {
    pub rfid: String,
    pub location: usize,
    pub time: Option<String>,
}

impl TimeStamp {
    pub fn new(rfid: String, location: usize, time: Option<String>) -> Self {
        Self {
            rfid,
            location,
            time,
        }
    }
    pub fn get_test_timestamps_from_file() -> Result<Vec<TimeStamp>, serde_json::Error> {
        if let Ok(ts_str) = std::fs::read_to_string("seed_data/timestamps.json") {
            serde_json::from_str::<Vec<TimeStamp>>(&ts_str)
        } else {
            Err(serde_json::Error::custom("Unable to read file"))
        }
    }
}

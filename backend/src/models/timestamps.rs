use std::fmt::{Display, Formatter};

use actix_web::ResponseError;
use serde::{de::Error, Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Eq, PartialEq)]
pub struct PostTimestamp {
    pub rfid: String,
    pub location: usize,
}

impl PostTimestamp {
    pub fn new(rfid: String, location: usize) -> Self {
        Self { rfid, location }
    }
}

use chrono::NaiveDate;
use serde::Deserializer;

#[derive(Debug, Deserialize)]
pub struct RangeParams {
    #[serde(deserialize_with = "deserialize_date")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    pub end_date: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimestampResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<TimestampData>,
}

impl Display for TimestampResponse {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for TimestampResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub enum TimestampData {
    Get(Vec<TimeStamp>),
    Post(PostTimestamp),
}

impl TimestampResponse {
    pub fn from_ts(ts: &PostTimestamp) -> Self {
        Self {
            success: true,
            message: "Timestamp successfully retrieved".to_string(),
            data: Some(TimestampData::Post(ts.clone())),
        }
    }
    pub fn from_db(ts: Vec<TimeStamp>) -> Self {
        Self {
            success: true,
            message: "Timestamps successfully retrieved".to_string(),
            data: Some(TimestampData::Get(ts)),
        }
    }
    pub fn from_error(msg: &str) -> Self {
        Self {
            success: false,
            message: msg.to_string(),
            data: None,
        }
    }
}

// Deserialize date strings into NaiveDate
fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(serde::de::Error::custom)
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

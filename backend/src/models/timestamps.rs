use std::fmt::{Display, Formatter};

use actix_web::ResponseError;
use serde::{de::Error, Deserialize, Serialize};

use chrono::NaiveDate;
use serde::Deserializer;

use super::{locations::Location, residents::Resident};

#[derive(Debug, Deserialize)]
pub struct RangeParams {
    #[serde(deserialize_with = "deserialize_date")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    pub end_date: NaiveDate,
}

pub trait Serializable {}
impl Serializable for TimeStamp {}
impl Serializable for PostTimestamp {}
impl Serializable for Resident {}
impl Serializable for Location {}
impl Serializable for String {}
impl Serializable for ResidentTimestamp {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<Vec<T>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResidentTimestamp {
    pub resident: Resident,
    pub timestamp: TimeStamp,
}

impl<T: Serializable> Display for Response<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<T: Serializable + std::fmt::Debug> ResponseError for Response<T> {}

impl<T: Serializable + std::fmt::Debug> From<rusqlite::Error> for Response<T> {
    fn from(e: rusqlite::Error) -> Self {
        Self::from_error(e.to_string().as_str())
    }
}

impl<T: Serializable> From<ResidentTimestamp> for Response<T>
where
    T: From<ResidentTimestamp>,
{
    fn from(res_ts: ResidentTimestamp) -> Self {
        Self {
            success: true,
            message: "Resident Timestamp successfully retrieved".to_string(),
            data: Some(vec![T::from(res_ts)]),
        }
    }
}

impl<T: Serializable> From<TimeStamp> for Response<T>
where
    T: From<TimeStamp>,
{
    fn from(ts: TimeStamp) -> Self {
        Self {
            success: true,
            message: "Timestamp successfully retrieved".to_string(),
            data: Some(vec![T::from(ts)]),
        }
    }
}

impl From<TimeStamp> for Vec<TimeStamp> {
    fn from(ts: TimeStamp) -> Self {
        vec![ts]
    }
}

impl<T> From<Vec<TimeStamp>> for Response<T>
where
    T: Serializable + From<TimeStamp>,
{
    fn from(ts: Vec<TimeStamp>) -> Self {
        let data: Vec<T> = ts.into_iter().map(T::from).collect();

        Self {
            success: true,
            message: "Timestamps successfully retrieved".to_string(),
            data: Some(data),
        }
    }
}
impl<T: Serializable> From<Resident> for Response<T>
where
    T: From<Resident>,
{
    fn from(res: Resident) -> Self {
        Self {
            success: true,
            message: "Resident successfully retrieved".to_string(),
            data: Some(vec![T::from(res)]),
        }
    }
}
impl<T: Serializable> From<Vec<Resident>> for Response<T>
where
    T: From<Resident>,
{
    fn from(res: Vec<Resident>) -> Self {
        Self {
            success: true,
            message: "Residents successfully retrieved".to_string(),
            data: Some(res.into_iter().map(T::from).collect()),
        }
    }
}
impl From<Location> for Response<Location> {
    fn from(loc: Location) -> Self {
        Self {
            success: true,
            message: "Location successfully retrieved".to_string(),
            data: Some(vec![loc]),
        }
    }
}
impl From<Vec<Location>> for Response<Location> {
    fn from(loc: Vec<Location>) -> Self {
        Self {
            success: true,
            message: "Locations successfully retrieved".to_string(),
            data: Some(loc),
        }
    }
}

impl<T> Response<T>
where
    T: Serializable + std::fmt::Debug,
{
    pub fn from_success(msg: &str) -> Self {
        Self {
            success: true,
            message: msg.to_string(),
            data: None,
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

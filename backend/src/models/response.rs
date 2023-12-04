use super::locations::Location;
use super::residents::Resident;
use super::timestamps::{PostTimestamp, ResidentTimestamp, TimeStamp};
use actix_web::ResponseError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

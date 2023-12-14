use actix_web::error::BlockingError;
use actix_web::ResponseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateResident {
    pub rfid: Option<String>,
    pub name: Option<String>,
    pub doc: Option<String>,
    pub room: Option<String>,
    pub unit: Option<usize>,
    pub current_location: Option<usize>,
    pub level: Option<usize>,
}

use std::fmt::{Display, Formatter};

use chrono::NaiveDate;
use serde::Deserializer;

#[derive(Debug, Deserialize)]
pub struct PathParams {
    pub rfid: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub start_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    pub end_date: NaiveDate,
}

// Deserialize date strings into NaiveDate
fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResidentsError(pub String);
impl ResponseError for ResidentsError {}

impl Display for ResidentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occured on the input: {}", self.0)
    }
}
pub enum ErrorType {
    Delete,
    Store,
    Update,
    NotFound,
    Validation,
    Database,
    Custom(String),
}

impl std::convert::From<actix_web::error::BlockingError> for ResidentsError {
    fn from(e: BlockingError) -> Self {
        Self::new(e.to_string())
    }
}

impl ResidentsError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
    pub fn get(e: ErrorType) -> Self {
        match e {
            ErrorType::Delete => Self::new("Unable to delete Resident from database".to_string()),
            ErrorType::Store => Self::new("Unable to create Resident in database".to_string()),
            ErrorType::Update => Self::new("Unable to update Resident in database".to_string()),
            ErrorType::NotFound => Self::new("No Resident found with id".to_string()),
            ErrorType::Database => {
                Self::new("An error occurred with your query, please check your inputs".to_string())
            }
            ErrorType::Validation => {
                Self::new("A validation error occured on the input".to_string())
            }
            ErrorType::Custom(message) => Self::new(format!("An error occurred: {}", message)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Rfid {
    pub rfid: String,
}

impl Display for Rfid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rfid)
    }
}

use actix_web::error::BlockingError;
use actix_web::ResponseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Resident {
    pub rfid: String, // rfid_tag: 123455623562354
    pub name: String, // name: Last, First
    pub doc: String,  // doc: 123345
    pub room: String,
    pub unit: usize,
    pub current_location: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ResidentResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Vec<Resident>>,
}
impl ResponseError for ResidentResponse {}
impl Display for ResidentResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl ResidentResponse {
    pub fn from_resident(resident: &Resident) -> Self {
        Self {
            success: true,
            message: "Resident successfully retrieved".to_string(),
            data: Some(vec![resident.clone()]),
        }
    }
    pub fn from_success(s: &str) -> Self {
        Self {
            success: true,
            message: s.to_string(),
            data: None,
        }
    }
    pub fn from_vec(residents: Vec<Resident>) -> Self {
        Self {
            success: true,
            message: "Residents successfully retrieved".to_string(),
            data: Some(residents),
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

impl Resident {
    pub fn new(
        rfid: String,
        name: String,
        doc: String,
        room: String,
        unit: usize,
        current_location: usize,
    ) -> Self {
        Self {
            rfid,
            name,
            doc,
            room,
            unit,
            current_location,
        }
    }

    pub fn update_location(&mut self, new_location: usize) {
        self.current_location = if self.current_location == new_location {
            0
        } else {
            new_location
        }
    }

    pub fn get_test_residents_from_file() -> Vec<Resident> {
        let file = std::fs::read_to_string("seed_data/residents.json").unwrap();
        serde_json::from_str::<Vec<Resident>>(&file).unwrap()
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateResident {
    pub rfid: Option<String>,
    pub name: Option<String>,
    pub doc: Option<String>,
    pub room: Option<String>,
    pub unit: Option<usize>,
    pub current_location: Option<usize>,
}

impl UpdateResident {
    pub fn apply_to(self, resident: Resident) -> Resident {
        Resident {
            rfid: self.rfid.unwrap_or(resident.rfid),
            name: self.name.unwrap_or(resident.name),
            doc: self.doc.unwrap_or(resident.doc),
            room: self.room.unwrap_or(resident.room),
            unit: self.unit.unwrap_or(resident.unit),
            current_location: self.current_location.unwrap_or(resident.current_location),
        }
    }
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

impl std::convert::From<rusqlite::Error> for ResidentsError {
    fn from(e: rusqlite::Error) -> Self {
        Self::new(e.to_string())
    }
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

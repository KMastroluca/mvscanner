use crate::{
    database::db::{query, Pool, Query, QueryResult},
    models::residents::{Resident, UpdateResident},
};
use actix_web::{
    delete,
    error::BlockingError,
    get,
    http::{header, StatusCode},
    patch, post, web, HttpResponse, ResponseError,
};
use std::fmt::{Display, Formatter};

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize)]
pub struct PathParams {
    rfid: String,
    #[serde(deserialize_with = "deserialize_date")]
    start_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    end_date: NaiveDate,
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
    rfid: String,
}

impl Display for Rfid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rfid)
    }
}

#[get("/api/residents")]
pub async fn index(db: web::Data<Pool>) -> Result<HttpResponse, ResidentsError> {
    if let Ok(res) = query(&db, Query::IndexResidents).await {
        match res {
            QueryResult::Residents(residents) => Ok(HttpResponse::Ok()
                .insert_header(header::ContentType::json())
                .json(residents)),
            _ => Err(ResidentsError::get(ErrorType::Database)),
        }
    } else {
        Err(ResidentsError::get(ErrorType::Database))
    }
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}")]
pub async fn show(db: web::Data<Pool>, rfid: actix_web::web::Path<Rfid>) -> Result<HttpResponse, ResidentsError> {
    if let Ok(res) = query(&db, Query::ShowResident(rfid.rfid.clone())).await {
        match res {
            QueryResult::Resident(resident) => {
                Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(resident))
            }
            _ => Err(ResidentsError::get(ErrorType::Database)),
        }
        } else {
            Err(ResidentsError::get(ErrorType::Database))
    }
}

#[rustfmt::skip]
#[post("/api/residents")]
pub async fn store(db: web::Data<Pool>, resident: web::Json<Resident>) -> Result<HttpResponse, ResidentsError> {
    if let Ok(res) = query(&db, Query::StoreResident(&resident.into_inner())).await {
        match res {
            QueryResult::Success => {
                Ok(HttpResponse::Ok().status(StatusCode::CREATED).insert_header(header::ContentType::json()).json("Success"))
            }
            _ => Err(ResidentsError::get(ErrorType::Database))
        }
    } else {
        Err(ResidentsError::get(ErrorType::Database))
    }
}

#[rustfmt::skip]
#[delete("/api/residents/{rfid}")]
pub async fn destroy(db: web::Data<Pool>, rfid: web::Path<String>,) -> Result<HttpResponse, ResidentsError> {
    if let Ok(res) = query(&db, Query::DestroyResident(rfid.into_inner())).await {
        match res {
            QueryResult::Resident(resident) => {
                Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(resident))
            }
        _ => Err(ResidentsError::get(ErrorType::Database))
        }
    } else {
         Err(ResidentsError::get(ErrorType::Database))
    }
}

#[rustfmt::skip]
#[patch("/api/residents/{rfid}")]
pub async fn update(db: web::Data<Pool>, rfid: web::Path<String>, resident: web::Json<UpdateResident>) -> Result<HttpResponse, ResidentsError> {
    match query(&db, Query::ShowResident(rfid.into_inner())).await {
        Ok(QueryResult::Resident(res)) => {
            let updated = resident.into_inner().apply_to(res.clone());
            match query(&db, Query::UpdateResident(&updated)).await {
                Ok(QueryResult::Resident(updated_resident)) => {
                    Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(updated_resident))
                }
                _ => Err(ResidentsError::get(ErrorType::Database)),
            }
        }
        _ => Err(ResidentsError::get(ErrorType::Database)),
    }
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}/timestamps")]
pub async fn show_resident_timestamps(db: web::Data<Pool>, rfid: actix_web::web::Path<Rfid>) -> Result<HttpResponse, ResidentsError> {
    if let Ok(ts) = query(&db, Query::ShowResidentTimestamps(rfid.rfid.clone())).await {
        match ts {
            QueryResult::TimeStamps(ts) => {
                Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(ts))
        }
        _ => Err(ResidentsError::get(ErrorType::Database)),
    }
    } else {
    Err(ResidentsError::get(ErrorType::Database))
    }
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}/timestamps/{start_date}/{end_date}")]
pub async fn show_resident_timestamps_range(db: web::Data<Pool>, rfid: actix_web::web::Path<PathParams>) -> Result<HttpResponse, ResidentsError> {
    let id = rfid.into_inner();
    let rfid = id.rfid;
    let start = id.start_date;
    let end = id.end_date;

    if let Ok(ts) = query(&db, Query::ShowResidentTimestampsRange(&rfid, &start, &end)).await {
        match ts {
            QueryResult::TimeStamps(ts) => {
                Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(ts))
            }
            _ => Err(ResidentsError::get(ErrorType::Database)),
        }
    } else {
        Err(ResidentsError::get(ErrorType::Database))
    }
}

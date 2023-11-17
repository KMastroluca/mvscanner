use crate::{
    database::db::{query, Pool, Query, QueryResult},
    models::{residents::Resident, timestamps::Range},
};
use actix_web::{
    delete, error::BlockingError, get, http::header, post, put, web, HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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
#[delete("/api/residents")]
pub async fn destroy(db: web::Data<Pool>, rfid: actix_web::web::Path<String>,) -> Result<HttpResponse, ResidentsError> {
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
#[put("/api/residents")]
pub async fn update(db: web::Data<Pool>, resident: web::Json<Resident>) -> Result<HttpResponse, ResidentsError> {
    if let Ok(res) = query(&db, Query::UpdateResident(&resident.into_inner())).await {
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
#[get("/api/residents/{rfid}/timestamps{range}")]
pub async fn show_resident_timestamps(db: web::Data<Pool>, rfid: actix_web::web::Path<Rfid>, range: web::Json<Range>) -> Result<HttpResponse, ResidentsError> {
    if let Ok(ts) = query(&db, Query::ShowResidentTimestamps(rfid.rfid.clone(), &range.into_inner())).await {
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

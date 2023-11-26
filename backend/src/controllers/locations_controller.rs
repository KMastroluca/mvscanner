use std::fmt::{Display, Formatter};


use crate::database::db::{query, Pool, Query, QueryResult};
use crate::models::locations::{Location, LocationsResponse};
use crate::models::residents::ResidentResponse;
use crate::models::timestamps::TimestampResponse;
use actix_web::http::{header, StatusCode};
use actix_web::{get, post, web, HttpResponse};
use actix_web::{Responder, ResponseError};
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct LocationRange {
    location_id: usize,
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
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Id {
    pub location_id: usize,
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location Id: {}", self.location_id)
    }
}

#[derive(Debug, Deserialize)]
pub struct LocationsError(pub String);
impl ResponseError for LocationsError {}

impl std::fmt::Display for LocationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occured on the input: {}", self.0)
    }
}

// index all locations
#[rustfmt::skip]
#[get("/api/locations")]
pub async fn index(db: web::Data<Pool>) -> impl Responder {
    log::info!("GET: locations controller");
    if let Ok(res) = query(&db, Query::IndexLocations).await {
        match res {
        QueryResult::Locations(locations) => {
                let response = LocationsResponse::from_locations(locations);
            Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
            } 
        _ => {
    let response = LocationsResponse::from_error("Error retrieving locations");
            Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
    } 
        }
    } else {
        Err(LocationsError("Unable to retrieve locations".to_string()))
    }
}
// add a new location
#[rustfmt::skip]
#[post("/api/locations")]
pub async fn store(db: web::Data<Pool>, loc: web::Json<Location>) -> Result<HttpResponse, LocationsError> {
    log::info!("POST: locations controller");
    if let Ok(QueryResult::Success) = query(&db, Query::StoreLocation(&loc.into_inner())).await {
        let response = LocationsResponse::from_success("Location successfully added");
        Ok(HttpResponse::Ok().status(StatusCode::CREATED).insert_header(header::ContentType::json()).json(response))
    } else {
        Err(LocationsError("Unable to add location".to_string()))
    }
}

// Get location name from ID
#[get("/api/locations/{location_id}")]
pub async fn show(db: web::Data<Pool>, id: web::Path<Id>) -> Result<HttpResponse, LocationsError> {
    log::info!("GET: locations controller with id: {}", id.location_id);
    if let Ok(QueryResult::Location(loc)) = query(&db, Query::ShowLocation(id.location_id)).await {
        let loc = LocationsResponse::from_location(loc);
        Ok(HttpResponse::Ok()
            .insert_header(header::ContentType::json())
            .json(loc))
    } else {
        Err(LocationsError("Unable to retrieve location".to_string()))
    }
}

// include range in url to show timestamps from /start/end
#[rustfmt::skip]
#[get("/api/locations/{location_id}/timestamps/{start_date}/{end_date}")]
pub async fn show_location_timestamps_range(db: web::Data<Pool>, id: web::Path<LocationRange>) -> Result<HttpResponse, LocationsError> {
    let loc_range = id.into_inner();
    log::info!("GET: Locations controller timestamps with range for ID");
    if let Ok(QueryResult::TimeStamps(ts)) = query(&db, Query::ShowLocationTimestampsRange(loc_range.location_id, &loc_range.start_date, &loc_range.end_date)).await {
        let response: TimestampResponse = ts.into();
        Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
    } else {
        Err(LocationsError("Unable to retrieve timestamps".to_string()))
    }
}

// show timestamps from today for a location
#[rustfmt::skip]
#[get("/api/locations/{location_id}/timestamps")]
pub async fn show_location_timestamps(db: web::Data<Pool>, id: web::Path<Id>) -> impl Responder {
    let id = id.into_inner().location_id;
    log::info!("GET: Locations controller timestamps for ID");
    if let Ok(QueryResult::TimeStamps(ts)) = query(&db, Query::ShowLocationTimestamps(id)).await {
        let response: TimestampResponse = ts.into();
        Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
    } else {
        Err(LocationsError("Unable to retrieve timestamps".to_string()))
    }
}

// show all residents for a given location
#[rustfmt::skip]
#[get("/api/locations/{location_id}/residents")]
pub async fn show_location_residents(db: web::Data<Pool>, id: web::Path<Id>) -> Result<HttpResponse, LocationsError> {
    let id = id.into_inner().location_id;
    log::info!("GET: Locations controller residents for ID");
    if let Ok(QueryResult::Residents(res)) = query(&db, Query::ShowLocationResidents(id)).await {
        let response: ResidentResponse = res.into();
        Ok(HttpResponse::Ok()
            .insert_header(header::ContentType::json())
            .json(response))
    } else {
        Err(LocationsError("Unable to retrieve residents".to_string()))
    }
}

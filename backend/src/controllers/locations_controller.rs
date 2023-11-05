use std::fmt::{Display, Formatter};

use crate::database::db::DB;
use crate::models::locations::Location;
use crate::models::timestamps::Range;
use actix_web::ResponseError;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

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

#[rustfmt::skip]
#[get("/api/locations")]
pub async fn index(db: web::Data<DB>) -> Result<HttpResponse, LocationsError> {
    log::info!("GET: locations controller");
    let locations = Location::index(&db.into_inner());
    if locations.is_empty() {
        Err(LocationsError("No locations found".to_string()))
    } else {
        Ok(HttpResponse::Ok().json(locations))
    }
}

#[rustfmt::skip]
#[post("/api/locations")]
pub async fn store(db: web::Data<&DB>, loc: web::Json<Location>) -> Result<HttpResponse, LocationsError> {
    log::info!("POST: locations controller");
    let loc = loc.into_inner();
    if let Ok(loc) = Location::store(&db.into_inner(), &loc) {
        Ok(HttpResponse::Ok().json(loc))
    } else {
        Err(LocationsError("Unable to store location".to_string()))
    }
}

#[get("/api/locations/{location_id}")]
pub async fn show(db: web::Data<&DB>, id: web::Path<Id>) -> Result<HttpResponse, LocationsError> {
    let id = id.into_inner();
    log::info!("GET: locations controller with id: {}", id.location_id);
    let loc = Location::show(&db.into_inner(), id);
    if let Some(loc) = loc {
        Ok(HttpResponse::Ok().json(loc))
    } else {
        Err(LocationsError(format!(
            "No location found with id: {}",
            id.location_id
        )))
    }
}

#[rustfmt::skip]
#[get("/api/locations/{location_id}/timestamps")]
pub async fn show_location_range(db: web::Data<&DB>, id: web::Path<Id>, range: web::Json<Range>) -> Result<HttpResponse, LocationsError> {
    let id = id.into_inner();
    let locations =  Location::show_location_timestamps_range(&db.into_inner(), id.location_id, &range.into_inner());
    if locations.is_empty() {
        Err(LocationsError("No timestamps found for that location/range".to_string())) 
        } else {
        Ok(HttpResponse::Ok().json(locations))
    }
}

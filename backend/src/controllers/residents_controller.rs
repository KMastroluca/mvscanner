use crate::{
    database::db::DB,
    models::{
        residents::Resident,
        timestamps::{Range, TimeStamp},
    },
};
use actix_web::{delete, get, http::header, post, put, web, HttpResponse, ResponseError};
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
pub async fn index(db: web::Data<&DB>) -> Result<HttpResponse, ResidentsError> {
    let residents = Resident::index(&db.into_inner());
    if residents.is_empty() {
        return Err(ResidentsError("No Residents found in database".to_string()));
    }
    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .json(residents))
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}")]
pub async fn show(db: web::Data<&DB>, rfid: actix_web::web::Path<Rfid>) -> Result<HttpResponse, ResidentsError> {
    let id = rfid.into_inner();
    if let Some(resident) = Resident::show(&id.rfid, &db.into_inner()) {
    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType::json())
        .json(resident))
    } else {
        Err(ResidentsError(format!("No Resident found with id: {}", id.rfid)))
    }
}

#[rustfmt::skip]
#[post("/api/residents")]
pub async fn store(db: web::Data<&DB>, resident: web::Json<Resident>) -> Result<HttpResponse, ResidentsError> {
    let resident = resident.into_inner();
    if Resident::store(&resident, &db.into_inner()).is_ok() {
        Ok(HttpResponse::Ok().json("{'Success': 'true'}"))
    } else {
        Err(ResidentsError("Unable to store resident".to_string()))
    }
}

#[rustfmt::skip]
#[delete("/api/residents")]
pub async fn destroy(db: web::Data<&DB>, rfid: actix_web::web::Path<String>,) -> Result<HttpResponse, ResidentsError> {
    if Resident::delete(&rfid, &db.into_inner()).is_ok() {
    Ok(HttpResponse::Ok()
        .insert_header(header::ContentType::json()).json("{ 'success': 'true' }"))
    } else {
        Err(ResidentsError("Unable to delete resident".to_string()))
    }
}

#[rustfmt::skip]
#[put("/api/residents")]
pub async fn update(db: web::Data<&DB>, resident: web::Json<Resident>) -> Result<HttpResponse, ResidentsError> {
    let resident = resident.into_inner();
    if Resident::update(&resident, &db.into_inner()).is_ok() {
        Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json("Success"))
    } else {
        Err(ResidentsError(
            "Unable to update Resident in database".to_string(),
        ))
    }
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}/timestamps{range}")]
pub async fn show_resident_timestamps(db: web::Data<&DB>, rfid: actix_web::web::Path<Rfid>, range: web::Json<Range>) -> Result<HttpResponse, ResidentsError> {
    let ts = TimeStamp::show_range_resident(&db.into_inner(), &rfid.into_inner().rfid, &range.into_inner());
    if ts.is_empty() {
         Err(ResidentsError("No timestamps found for resident".to_string()))
    } else {
        Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(ts))
    }
}

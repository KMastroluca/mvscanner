use super::timestamps_controller::FilterOpts;
use crate::app_config::DB;
use crate::models::response::Response;
use actix_web::http::header::ContentType;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use chrono::{Days, NaiveDate};
use entity::{
    locations::{self, Entity as Locations},
    residents::Entity as Residents,
    timestamps::Entity as Timestamps,
};
use entity::{residents, timestamps};
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, Set};
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct LocationRange {
    location_id: usize,
    #[serde(deserialize_with = "deserialize_date")]
    start_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_date")]
    end_date: NaiveDate,
}
#[derive(Debug, Deserialize)]
pub struct Params {
    pub current: Option<bool>,
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
pub async fn index(db: web::Data<DB>) -> impl Responder {
    let db = &db.0;
    log::info!("GET: locations controller");
    if let Ok(location) = Locations::find().all(db).await {
    let response: Response<locations::Model> = Response::from(location);
    HttpResponse::Ok().insert_header(ContentType::json()).json(response)
    } else {
        let response = Response::<String>::from_error("Error retrieving locations");
        HttpResponse::Ok().insert_header(ContentType::json()).json(response)
    }
}

// add a new location
#[rustfmt::skip]
#[post("/api/locations")]
pub async fn store(db: web::Data<DB>, loc: web::Json<locations::Model>) -> impl Responder {
    let db = &db.0;
    log::info!("POST: locations controller");
    let loc = loc.into_inner();
    let location = locations::ActiveModel {
        id: Set(loc.id),
        name: Set(loc.name),
        ..Default::default()
    };
    if Locations::insert(location).exec(db).await.is_ok() {
    let resp: Response<String> = Response::from_success("Location successfully added");
    HttpResponse::Ok().insert_header(ContentType::json()).json(resp)
    } else {
    HttpResponse::Ok().insert_header(ContentType::json()).json(Response::<String>::from_error("Error adding location"))
    }
}

// Get location name from ID
#[get("/api/locations/{location_id}")]
pub async fn show(db: web::Data<DB>, id: web::Path<Id>) -> impl Responder {
    let db = &db.0;
    let id = id.into_inner().location_id;
    log::info!("GET: Locations Show: {}", id);
    if let Some(location) = Locations::find_by_id(id as i32).one(db).await.unwrap() {
        let response: Response<entity::locations::Model> = Response::from(location);
        HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(response)
    } else {
        let response = Response::<String>::from_error("Error retrieving location");
        HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(response)
    }
}

// include range in url to show timestamps from /start/end
#[rustfmt::skip]
#[get("/api/locations/{location_id}/timestamps/{start_date}/{end_date}")]
pub async fn show_location_timestamps_range(db: web::Data<DB>, id: web::Path<LocationRange>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let db = &db.0;
    let loc_range = id.into_inner();
    let timestamps = Timestamps::find().filter(
        timestamps::Column::Location
            .eq(loc_range.location_id as i32)
            .and(timestamps::Column::Ts.between(loc_range.start_date.and_hms_opt(0, 0, 0), loc_range.end_date.and_hms_opt(23, 59, 59))),
    ).order_by_desc(timestamps::Column::Ts).all(db).await?;
    let response: Response<timestamps::Model> = Response::from(timestamps);
    Ok(HttpResponse::Ok().insert_header(ContentType::json()).json(response))
}

// show timestamps from today for a location
#[rustfmt::skip]
#[get("/api/locations/{location_id}/timestamps")]
pub async fn show_location_timestamps(db: web::Data<DB>, id: web::Path<Id>, uni: web::Query<FilterOpts>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let db = &db.0;
    let id = id.into_inner().location_id;
    let mut hshset = std::collections::HashSet::new();
    let result: Vec<timestamps::Model> = Timestamps::find()
        .filter(
            timestamps::Column::Location.eq(id as i32)
                .and(timestamps::Column::Ts.between(chrono::offset::Local::now().checked_sub_days(Days::new(1)), Some(chrono::offset::Local::now())))
        ).order_by_desc(timestamps::Column::Ts)
        .all(db)
        .await?;
        let mut res = Vec::new();
    if let Some(true) = uni.into_inner().unique {
        for ts in result {
            if hshset.insert(ts.rfid.clone()) {
           res.push(ts);
            } else {
                continue;
            }
        }
        return Ok(HttpResponse::Ok().insert_header(ContentType::json()).json(Response::<timestamps::Model>::from(res)));
    } else {
       return Ok(HttpResponse::Ok().insert_header(ContentType::json()).json(Response::from(result)));
    }
}

// show all residents for a given location
#[rustfmt::skip]
#[get("/api/locations/{location_id}/residents")]
pub async fn show_location_residents(db: web::Data<DB>, id: web::Path<Id>, curr: web::Query<Params>) -> Result<HttpResponse, Box<dyn std::error::Error>>  {
    let db = &db.0;
    let id = id.into_inner().location_id;
    if curr.into_inner().current.is_some_and(|c| c) {
        let residents: Vec<residents::Model> = Residents::find()
            .filter(residents::Column::CurrentLocation.eq(id as i32))
            .all(db)
            .await?;
        let response: Response<residents::Model> = Response::from(residents);
        Ok(HttpResponse::Ok().insert_header(ContentType::json()).json(response))
    } else {
        let residents: Vec<residents::Model> = Residents::find()
            .filter(residents::Column::Unit.eq(id as i32))
            .all(db)
            .await?;
        let response: Response<residents::Model> = Response::from(residents);
        Ok(HttpResponse::Ok().insert_header(ContentType::json()).json(response))
    }
}

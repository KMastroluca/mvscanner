use crate::app_config::DB;
use crate::models::residents::UpdateResident;
use crate::models::{
    residents::{PathParams, Rfid},
    response::Response,
};
use actix_web::Responder;
use actix_web::{
    delete, get,
    http::{header, StatusCode},
    patch, post, web, HttpResponse,
};
use chrono::Days;
use entity::{
    residents::{self, Entity as Resident},
    timestamps,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

#[get("/api/residents")]
pub async fn index(db: web::Data<DB>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let db = &db.0;
    if let Ok(residents) = Resident::find().all(db).await {
        let response = Response::from(residents);
        Ok(HttpResponse::Ok()
            .insert_header(header::ContentType::json())
            .json(response))
    } else {
        Ok(HttpResponse::Ok().body("Error retrieving residents"))
    }
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}")]
pub async fn show(db: web::Data<DB>, rfid: actix_web::web::Path<Rfid>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let db = &db.0;
    let rfid = rfid.into_inner().rfid;
    if let Ok(resident) = Resident::find().filter(residents::Column::Rfid.eq(rfid.clone())).one(db).await {
    if resident.is_none() {
        let error = Response::<String>::from_error("Error retrieving residents");
        return Ok(HttpResponse::Ok()
            .insert_header(header::ContentType::json())
            .json(error));
    }
    let response: Response<residents::Model> = Response::from(resident.unwrap());
    Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
    } else {
        let response = Response::<String>::from_error("Error retrieving residents");
        Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
    }
}

#[rustfmt::skip]
#[post("/api/residents")]
pub async fn store(db: web::Data<DB>, resident: web::Json<residents::Model>) -> impl Responder {
    let db = &db.0;
    let resident = resident.into_inner();
    let resident = residents::ActiveModel {
        rfid: Set(resident.rfid),
        name: Set(resident.name),
        doc: Set(resident.doc),
        room: Set(resident.room),
        unit: Set(resident.unit),
        current_location: Set(resident.current_location),
        level: Set(resident.level),
        ..Default::default()
    };
    if  Resident::insert(resident).exec(db).await.is_ok() {
        HttpResponse::Ok().insert_header(header::ContentType::json()).json(Response::<String>::from_success("Resident successfully added"))
    } else {
    HttpResponse::Ok().insert_header(header::ContentType::json()).json(Response::<String>::from_error("Error adding resident"))
    }
}

#[rustfmt::skip]
#[delete("/api/residents/{rfid}")]
pub async fn destroy(db: web::Data<DB>, rfid: web::Path<String>,) -> impl Responder {
    let db = &db.0;
    let rfid = rfid.into_inner();
    if let Ok(resident) = Resident::find().filter(residents::Column::Rfid.eq(rfid.clone())).one(db).await {
    let resident: residents::ActiveModel = resident.unwrap().into();
    match resident.delete(db).await {
        Ok(_) => 
    HttpResponse::Ok().status(StatusCode::NO_CONTENT).body(format!("Deleted resident: {}", rfid)),
    Err(e) => HttpResponse::Ok().body(format!("Error deleting resident: {}", e))
    }
    } else {
        HttpResponse::Ok().insert_header(header::ContentType::json()).json(Response::<String>::from_error("Error deleting resident"))
    }
}

#[rustfmt::skip]
#[patch("/api/residents/{rfid}")]
pub async fn update(db: web::Data<DB>, rfid: actix_web::web::Path<Rfid>, resident: web::Json<UpdateResident>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
     let db = &db.0;
    let rfid = rfid.into_inner().rfid;
    let resident = resident.into_inner();
    if let Ok(to_update) = Resident::find().filter(residents::Column::Rfid.eq(rfid.clone())).one(db).await {
    if to_update.is_none() {
        let error = Response::<String>::from_error("Error retrieving resident");
        return Ok(HttpResponse::Ok()
            .insert_header(header::ContentType::json())
            .json(error));
    } else {
        let mut to_update: residents::ActiveModel = to_update.unwrap().into();
        to_update.rfid = Set(resident.rfid.unwrap_or_else(|| to_update.rfid.unwrap()));
        to_update.name = Set(resident.name.unwrap_or_else(|| to_update.name.unwrap()));
        to_update.room = Set(resident.room.unwrap_or_else(|| to_update.room.unwrap()));
        to_update.unit = Set(resident.unit.unwrap_or_else(|| to_update.unit.unwrap() as usize) as i32);
        to_update.current_location = Set(resident.current_location.unwrap_or_else(|| to_update.current_location.unwrap() as usize) as i32);
        to_update.level = Set(resident.level.unwrap_or_else(|| to_update.level.unwrap() as usize) as i32);
        to_update.save(db).await?;
        let response: Response<String> = Response::from_success("Resident Updated Successfully");
        Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
    }

} else {
        Ok(HttpResponse::Ok().body("Error updating resident"))
    }
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}/timestamps")]
pub async fn show_resident_timestamps(db: web::Data<DB>, rfid: actix_web::web::Path<Rfid>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let db = &db.0;
    let rfid = rfid.into_inner().rfid;
    if let Ok(ts) = timestamps::Entity::find()
        .filter(timestamps::Column::Rfid.contains(rfid))
        .filter(timestamps::Column::Ts.between(chrono::Local::now().checked_sub_days(Days::new(1)), Some(chrono::Local::now())))
        .all(db).await {
    let response: Response<timestamps::Model> = Response::from(ts);
    Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
     } else {
    let response = Response::<String>::from_error("Error retrieving timestamps");
    Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
    }
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}/timestamps/{start_date}/{end_date}")]
pub async fn show_resident_timestamps_range(db: web::Data<DB>, rfid: actix_web::web::Path<PathParams>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let db = &db.0;
    let id = rfid.into_inner();
    let rfid = id.rfid;
    let start = id.start_date;
    let end = id.end_date;
    if let Ok(ts) = timestamps::Entity::find()
        .filter(timestamps::Column::Rfid.contains(rfid))
        .filter(timestamps::Column::Ts.between(start, end))
        .all(db).await {
let response: Response<timestamps::Model> = Response::from(ts);
    Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
    } else {
        Ok(HttpResponse::Ok().body("Error retrieving timestamps"))
    }
}

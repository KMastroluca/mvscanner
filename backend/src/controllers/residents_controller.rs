use crate::{database::db::DB, models::residents::Resident};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/api/residents")]
pub async fn index(db: web::Data<&DB>) -> impl Responder {
    let residents = Resident::index(&db.into_inner());
    HttpResponse::Ok().json(residents)
}

#[get("/api/residents/{rfid}")]
pub async fn show(db: web::Data<&DB>, rfid: actix_web::web::Path<String>) -> impl Responder {
    if let Some(resident) = Resident::show(&rfid.into_inner(), &db.into_inner()) {
        HttpResponse::Ok().json(resident)
    } else {
        HttpResponse::BadRequest().body("Unable to deserialize resident")
    }
}

#[post("/api/residents/{payload}")]
pub async fn store(db: web::Data<&DB>, resident: web::Json<Resident>) -> impl Responder {
    let resident = resident.into_inner();
    if let Ok(_) = Resident::store(&resident, &db.into_inner()) {
        HttpResponse::Ok().json("Success")
    } else {
        HttpResponse::BadRequest().body("Unable to store resident")
    }
}

#[delete("/api/residents/{rfid}")]
pub async fn destroy(db: web::Data<&DB>, rfid: actix_web::web::Path<String>) -> impl Responder {
    if let Ok(_) = Resident::delete(&rfid, &db.into_inner()) {
        HttpResponse::Ok().json("Success")
    } else {
        HttpResponse::BadRequest().body("Unable to delete resident")
    }
}

#[put("/api/residents/{payload}")]
pub async fn update(db: web::Data<&DB>, resident: web::Json<Resident>) -> impl Responder {
    let resident = resident.into_inner();
    if let Ok(_) = Resident::update(&resident, &db.into_inner()) {
        HttpResponse::Ok().json("Success")
    } else {
        HttpResponse::BadRequest().body("Unable to update resident")
    }
}

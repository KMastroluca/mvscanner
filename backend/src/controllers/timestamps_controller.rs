use crate::{
    database::db::DB,
    models::timestamps::{Range, TimeStamp},
};
use actix_web::{get, post, web, HttpResponse, Responder};

/// GET: /api/timestamps  DEFAULT: Today
#[get("/api/timestamps")]
pub async fn index_timestamps(db: web::Data<&DB>) -> impl Responder {
    let timestamp = TimeStamp::index(&db.into_inner());
    HttpResponse::Ok().json(timestamp)
}

/// POST: /api/timestamps/{timestamp}
#[post("/api/timestamps")]
pub async fn store_timestamp(db: web::Data<&DB>, ts: web::Json<TimeStamp>) -> impl Responder {
    if let Ok(timestamp) = TimeStamp::store(&db.into_inner(), &ts.into_inner()) {
        HttpResponse::Ok().json(timestamp)
    } else {
        HttpResponse::BadRequest().body("Unable to process request")
    }
}

/// GET: /api/timestamps/{range}
#[get("/api/timestamps")]
pub async fn show_range(db: web::Data<&DB>, range: web::Json<Range>) -> impl Responder {
    HttpResponse::Ok().json(TimeStamp::index_range(
        &db.into_inner(),
        &range.into_inner(),
    ))
}

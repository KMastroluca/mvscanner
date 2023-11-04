use crate::models::timestamp::Timestamp;
use actix_web::{web, HttpResponse, Responder};

/// GET: /api/timestamps
pub fn index_timestamps(ts_request: Option<web::Path<String>>) -> impl Responder {
    let timestamp = Timestamp::all();
    HttpResponse::Ok().body(format!("Timestamp: {}", timestamp))
}

/// POST: /api/timestamps/{timestamp}
pub fn store_timestamp(ts_request: web::Path<String>) -> impl Responder {
    let timestamp = Timestamp::new();
    HttpResponse::Ok().body(format!("Timestamp: {}", timestamp))
}

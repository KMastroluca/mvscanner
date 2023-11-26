use crate::{
    database::db::{query, Pool, Query, QueryResult},
    models::timestamps::{RangeParams, TimeStamp, TimestampResponse},
};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, web, HttpResponse, Responder,
};

/// GET: /api/timestamps  DEFAULT: Today
#[get("/api/timestamps")]
pub async fn index_timestamps(db: web::Data<Pool>) -> impl Responder {
    if let Ok(QueryResult::TimeStamps(ts)) = query(&db, Query::IndexTimestamps).await {
        let response: TimestampResponse = ts.into();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(response)
    } else {
        let error = TimestampResponse::from_error("Error Retrieving timestamps");
        HttpResponse::Ok().json(error)
    }
}

/// POST: /api/timestamps/{timestamp}
#[rustfmt::skip]
#[post("/api/timestamps")]
pub async fn store_timestamp(db: web::Data<Pool>, ts: web::Json<TimeStamp>) -> impl Responder {
    let ts = ts.into_inner();
    log::info!("Storing timestamp: {:?}", ts);
    if let Ok(QueryResult::PostTimestamp(timestamp)) = query(&db, Query::StoreTimestamp(&ts)).await {
        let res: TimestampResponse = timestamp.into();
        HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .insert_header(ContentType::json())
            .json(res)
    } else {
        let error = TimestampResponse::from_error("Error storing timestamp");
        HttpResponse::from_error(error)
    }
}

/// GET: /api/timestamps/{start}/{end}
#[get("/api/timestamps/{start_date}/{end_date}")]
#[rustfmt::skip]
pub async fn show_range(db: web::Data<Pool>, range: web::Path<RangeParams>) -> impl Responder {
    let range = &range.into_inner();
    if let Ok(QueryResult::TimeStamps(ts)) = query(&db, Query::ShowTimestamps(&range.start_date, &range.end_date)).await {
        let response: TimestampResponse = ts.into(); 
        HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(response)
    } else {
        let resp = TimestampResponse::from_error("Error retrieving timestamps");
        HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(resp)
    }
}

use crate::{
    database::db::{query, Pool, Query, QueryResult},
    models::timestamps::{Range, TimeStamp},
};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};

/// GET: /api/timestamps  DEFAULT: Today
#[get("/api/timestamps")]
pub async fn index_timestamps(db: web::Data<Pool>) -> impl Responder {
    if let Ok(QueryResult::TimeStamps(ts)) = query(&db, Query::IndexTimestamps).await {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(ts)
    } else {
        HttpResponse::BadRequest().body("Unable to process request")
    }
}

/// POST: /api/timestamps/{timestamp}
#[post("/api/timestamps")]
pub async fn store_timestamp(db: web::Data<Pool>, ts: web::Json<TimeStamp>) -> impl Responder {
    if let Ok(QueryResult::Success) = query(&db, Query::StoreTimestamp(&ts.into_inner())).await {
        HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json("Timestamp stored successfully")
    } else {
        HttpResponse::BadRequest().body("Unable to process request")
    }
}

/// GET: /api/timestamps/{range}
#[get("/api/timestamps")]
pub async fn show_range(db: web::Data<Pool>, range: web::Json<Range>) -> impl Responder {
    if let Ok(QueryResult::TimeStamps(ts)) =
        query(&db, Query::ShowTimestamps(&range.into_inner())).await
    {
        HttpResponse::Ok().json(ts)
    } else {
        HttpResponse::BadRequest().body("Unable to process request")
    }
}

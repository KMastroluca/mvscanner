use crate::{
    database::db::{query, Pool, Query, QueryResult},
    models::timestamps::PostTimestamp,
};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, web, HttpResponse, Responder,
};

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct RangeParams {
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
pub async fn store_timestamp(db: web::Data<Pool>, ts: web::Json<PostTimestamp>) -> impl Responder {
    let ts = ts.into_inner();
    log::info!("Storing timestamp: {:?}", ts);
    if let Ok(QueryResult::Success) = query(&db, Query::StoreTimestamp(&ts)).await {
        HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .insert_header(ContentType::json())
            .json("SUCCESS: Timestamp stored successfully")
    } else {
        HttpResponse::BadRequest().body("Unable to process request")
    }
}

/// GET: /api/timestamps/{start}/{end}
#[get("/api/timestamps/{start_date}/{end_date}")]
#[rustfmt::skip]
pub async fn show_range(db: web::Data<Pool>, range: web::Path<RangeParams>) -> impl Responder {
    let range = &range.into_inner();
    if let Ok(QueryResult::TimeStamps(ts)) = query(&db, Query::ShowTimestamps(&range.start_date, &range.end_date)).await {
        HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(ts)
    } else {
        HttpResponse::BadRequest().body("Unable to process request")
    }
}

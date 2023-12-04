use crate::{
    database::db::{query, Pool, Query, QueryResult},
    models::timestamps::{PostTimestamp, RangeParams, ResidentTimestamp, Response, TimeStamp},
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
        let response: Response<TimeStamp> = ts.into();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(response)
    } else {
        let error: Response<String> = Response::from_error("Error Retrieving timestamps");
        HttpResponse::Ok().json(error)
    }
}

/// POST: /api/timestamps/{timestamp}
#[rustfmt::skip]
#[post("/api/timestamps")]
pub async fn store_timestamp(db: web::Data<Pool>, ts: web::Json<PostTimestamp>) -> impl Responder {
    let ts = ts.into_inner();
    log::info!("Storing timestamp: {:?}", ts);
    if let Ok(mut res) = query(&db, Query::ShowResident(ts.clone().rfid.as_str())).await {
        match res {
            QueryResult::Resident(ref mut resident) => {
            resident.update_location(ts.location);
                if let Ok(QueryResult::Success) = query(&db, Query::UpdateResidentLocation(resident)).await {
                    if let Ok(QueryResult::Success) = query(&db, Query::StoreTimestamp(&ts.clone().into())).await {
                        let res: Response<ResidentTimestamp> = Response::from(ResidentTimestamp {
                            resident: resident.clone(), timestamp:TimeStamp::new(ts.rfid.clone(), resident.current_location, None)});
                        HttpResponse::Ok()
                            .status(StatusCode::CREATED)
                            .insert_header(ContentType::json())
                            .json(res)
                    } else {
                        let error: Response<String> = Response::from_error("Error storing timestamp");
                        HttpResponse::from_error(error)
                    }
                } else {
                    let error: Response<String> = Response::from_error("Error updating resident location");
                    HttpResponse::Ok().json(error)
                }
                    }
            _ => {
                let error: Response<String> = Response::from_error("Error retrieving resident");
                HttpResponse::Ok().json(error)
            }
        }
    } else {
        let repsonse: Response<String> = Response::from_error("Please add resident to the system first");
        HttpResponse::Ok().json(repsonse)
    }
}

/// GET: /api/timestamps/{start}/{end}
#[get("/api/timestamps/{start_date}/{end_date}")]
#[rustfmt::skip]
pub async fn show_range(db: web::Data<Pool>, range: web::Path<RangeParams>) -> impl Responder {
    let range = &range.into_inner();
    if let Ok(QueryResult::TimeStamps(ts)) = query(&db, Query::ShowTimestamps(&range.start_date, &range.end_date)).await {
        let response: Response<TimeStamp> = ts.into(); 
        HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(response)
    } else {
        let resp: Response<String> = Response::from_error("Error retrieving timestamps");
        HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(resp)
    }
}

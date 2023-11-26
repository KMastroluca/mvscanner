use crate::models::residents::{
    ErrorType, PathParams, Resident, ResidentResponse, ResidentsError, Rfid,
};
use crate::models::timestamps::TimestampResponse;
use crate::{
    database::db::{query, Pool, Query, QueryResult},
    models::residents::UpdateResident,
};
use actix_web::Responder;
use actix_web::{
    delete, get,
    http::{header, StatusCode},
    patch, post, web, HttpResponse,
};

#[get("/api/residents")]
pub async fn index(db: web::Data<Pool>) -> impl Responder {
    if let Ok(res) = query(&db, Query::IndexResidents).await {
        match res {
            QueryResult::Residents(residents) => {
                let response: ResidentResponse = residents.into();
                Ok(actix_web::HttpResponse::Ok()
                    .insert_header(header::ContentType::json())
                    .json(response))
            }
            _ => Ok(HttpResponse::Ok()
                .insert_header(header::ContentType::json())
                .json(ResidentResponse::from_error("Error retrieving residents"))),
        }
    } else {
        Err(ResidentsError::get(ErrorType::Database))
    }
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}")]
pub async fn show(db: web::Data<Pool>, rfid: actix_web::web::Path<Rfid>) -> impl Responder {
    if let Ok(res) = query(&db, Query::ShowResident(&rfid.into_inner().rfid)).await {
        match res {
            QueryResult::Resident(resident) => {
                let response: ResidentResponse = resident.into();
                Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
            }
            _ => {
                let error = ResidentResponse::from_error("Error retrieving resident");
                Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(error))
            }
        }
        } else {
            Err(ResidentsError::get(ErrorType::Database))
    }
}

#[rustfmt::skip]
#[post("/api/residents")]
pub async fn store(db: web::Data<Pool>, resident: web::Json<Resident>) -> impl Responder {
    if let Ok(res) = query(&db, Query::StoreResident(&resident.into_inner())).await {
        match res {
            QueryResult::Success => {
                let response = ResidentResponse::from_success("Resident successfully added");
                HttpResponse::Ok().status(StatusCode::CREATED).insert_header(header::ContentType::json()).json(response)
            }
            _ => {
                let err = ResidentResponse::from_error("Error creating resident");
                HttpResponse::Ok().insert_header(header::ContentType::json()).json(err)
                }
        }
    } else {
        let resp = ResidentResponse::from_error("Error creating resident");
         HttpResponse::from_error(resp)
    }
}

#[rustfmt::skip]
#[delete("/api/residents/{rfid}")]
pub async fn destroy(db: web::Data<Pool>, rfid: web::Path<String>,) -> impl Responder {
    if let Ok(res) = query(&db, Query::DestroyResident(rfid.into_inner())).await {
        match res {
            QueryResult::Success => {
                let response = ResidentResponse::from_success("Resident successfully deleted");
                HttpResponse::Ok().status(StatusCode::NO_CONTENT).insert_header(header::ContentType::json()).json(response)
            }
        _ => {
            let error = ResidentResponse::from_error("Error deleting resident");
                HttpResponse::from_error(error)
            }
        }
    } else {
        let resp = ResidentResponse::from_error("Error Deleting Resident");
         HttpResponse::from_error(resp)
    }
}

#[rustfmt::skip]
#[patch("/api/residents/{rfid}")]
pub async fn update(db: web::Data<Pool>, rfid: actix_web::web::Path<Rfid>, resident: web::Json<UpdateResident>) -> impl Responder {
    match query(&db, Query::ShowResident(&rfid.into_inner().rfid)).await {
        Ok(QueryResult::Resident(res)) => {
            log::info!("fetched resident for updating: {:?}", res);
            let updated = resident.into_inner().apply_to(res.clone());
            // We have to get the full resident from DB before we can update it
            // so we can accept a JSON with only the fields they wish to update
            match query(&db, Query::UpdateResident(&updated)).await {
                Ok(QueryResult::Success) => {
                    let updated_res: ResidentResponse = updated.into();
                    Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(updated_res))
                }
                _ => {
                    let error = ResidentResponse::from_error("Error updating resident");
                    Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(error))
                },
            }
        }
        _ => Err(ResidentsError::get(ErrorType::Database)),
    }
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}/timestamps")]
pub async fn show_resident_timestamps(db: web::Data<Pool>, rfid: actix_web::web::Path<Rfid>) -> impl Responder {
    if let Ok(ts) = query(&db, Query::ShowResidentTimestamps(rfid.rfid.clone())).await {
        match ts {
            QueryResult::TimeStamps(ts) => {
                let response: TimestampResponse = ts.into();
                Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
        }
        _ => {
                let error = ResidentResponse::from_error("Error retrieving resident timestamps");
                Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(error))
            }
        }
    } else {
    Err(ResidentsError::get(ErrorType::Database))
    }
}

#[rustfmt::skip]
#[get("/api/residents/{rfid}/timestamps/{start_date}/{end_date}")]
pub async fn show_resident_timestamps_range(db: web::Data<Pool>, rfid: actix_web::web::Path<PathParams>) -> impl Responder {
    let id = rfid.into_inner();
    let rfid = id.rfid;
    let start = id.start_date;
    let end = id.end_date;

    if let Ok(ts) = query(&db, Query::ShowResidentTimestampsRange(&rfid, &start, &end)).await {
        match ts {
            QueryResult::TimeStamps(ts) => {
                let response: TimestampResponse = ts.into();
                Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(response))
            }
            _ => {
                let error = ResidentResponse::from_error("Error retrieving resident timestamps");
                Ok(HttpResponse::Ok().insert_header(header::ContentType::json()).json(error))
            } 
        }
    } else {
        Err(ResidentsError::get(ErrorType::Database))
    }
}

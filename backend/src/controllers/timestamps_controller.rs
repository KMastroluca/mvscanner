use crate::{
    app_config::DB,
    models::response::Response,
    models::timestamps::{PostTimestamp, RangeParams, ResidentTimestamp},
};

use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
use chrono::Days;
use entity::{
    residents::{self, Entity as Resident},
    timestamps::{self, Entity as Timestamp},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect, Set,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FilterOpts {
    pub unique: Option<bool>,
}

/// GET: /api/timestamps?unique=true/false  DEFAULT: Today
#[rustfmt::skip]
#[get("/api/timestamps")]
pub async fn index_timestamps(db: web::Data<DB>, uni: web::Query<FilterOpts>) -> impl Responder {
    let db = &db.0;
    if let Some(true) = uni.into_inner().unique {
        if let Ok(ts) = Timestamp::find()
            .distinct_on([(timestamps::Entity, timestamps::Column::Rfid)])
            .filter(
                timestamps::Column::Ts.between(
                    chrono::Local::now().naive_local().date(),
                    chrono::Local::now()
                        .naive_local()
                        .date()
                        .checked_sub_days(Days::new(1))
                        .unwrap_or(chrono::Local::now().naive_local().date()),
                ),
            )
            .all(db)
            .await
        {
            let response: Response<timestamps::Model> = Response::from(ts);
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(response)
        } else {
            let resp: Response<String> = Response::from_error("Error retrieving timestamps");
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(resp)
        }
    } else if let Ok(ts) = Timestamp::find().all(db).await {
        let response: Response<timestamps::Model> = Response::from(ts);
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(response)
    } else {
        let resp: Response<String> = Response::from_error("Error retrieving timestamps");
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(resp)
    }
}

/// POST: /api/timestamps/{timestamp}
#[rustfmt::skip]
#[post("/api/timestamps")]
pub async fn store_timestamp(db: web::Data<DB>, timestamp_data: web::Json<PostTimestamp>) -> Result<HttpResponse, Box<dyn std::error::Error>>{
    let db = &db.0;
    let mut timestamp = timestamp_data.into_inner();
    match Resident::find_by_id(timestamp.rfid.clone()).one(db).await? {
        Some(resident) => {
             let mut resident = resident.into_active_model();
                if timestamp.location != resident.current_location.unwrap() as usize {
                    resident.current_location = Set(timestamp.location as i32);
                } else {
                    resident.current_location = Set(0);
                    timestamp.location = 0;
                }

                let resident = resident.save(db).await;
                let new_timestamp: timestamps::ActiveModel = timestamps::ActiveModel {
                        rfid: Set(timestamp.rfid.clone()),
                        location: Set(timestamp.location as i32),
                    ..Default::default()
                };
                new_timestamp.save(db).await?;
                let timestamp = timestamp.clone();
                let resident = resident.unwrap();
                let new_res = residents::Model {
                rfid: resident.rfid.into_value().unwrap().to_string(),
                name: resident.name.into_value().unwrap().to_string(),
                doc: resident.doc.into_value().unwrap().to_string(),
                unit: resident.unit.unwrap(),
                room: resident.room.into_value().unwrap().to_string(),
                current_location: resident.current_location.unwrap(),
                level: resident.level.unwrap(),
                };
            let new_ts = timestamps::Model {
                id: 0,
                rfid: timestamp.rfid,
                location: timestamp.location as i32,
                ts: chrono::Local::now().naive_local(),
            };
                let response = Response::<ResidentTimestamp>::from(ResidentTimestamp {
                    resident: new_res,
                    timestamp: new_ts,
                });
                Ok(HttpResponse::Ok().content_type(ContentType::json()).json(response))
        }
        None => {
            let error_resp: Response<String> = Response::from_error(&String::from("Error retrieving resident: Not found in system, please add Resident."));
            Ok(HttpResponse::Ok().content_type(ContentType::json()).json(error_resp))
        }
    }
}

/// GET: /api/timestamps/{start}/{end}
#[get("/api/timestamps/{start_date}/{end_date}")]
#[rustfmt::skip]
pub async fn show_range(db: web::Data<DB>, range: web::Path<RangeParams>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let db = &db.0;
    let range = range.into_inner();
    let time: Vec<entity::timestamps::Model>
    = Timestamp::find().filter(entity::timestamps::Column::Ts.between(range.start_date, range.end_date)).all(db).await?;
    let response = Response::<timestamps::Model>::from(time);
    Ok(HttpResponse::Ok().content_type(ContentType::json()).json(response))
}

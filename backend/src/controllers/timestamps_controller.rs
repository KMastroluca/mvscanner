use crate::{
    app_config::DB,
    models::response::Response,
    models::timestamps::{PostTimestamp, RangeParams, ResidentTimestamp},
};
use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
use chrono::{Duration, Local};
use entity::{
    residents::{self, Entity as Resident},
    timestamps::{self, Entity as Timestamp},
};
use reqwest::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, Set,
    TryIntoModel,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FilterOpts {
    pub unique: Option<bool>,
    pub per_page: Option<u64>,
}

/// GET: /api/timestamps?unique=true/false  DEFAULT: Today
#[rustfmt::skip]
#[get("/api/timestamps")]
pub async fn index_timestamps(db: web::Data<DB>, uni: web::Query<FilterOpts>) -> impl Responder {
    let page = uni.per_page.unwrap_or(0);
    let db = &db.0;
    let paginator = Timestamp::find()
    .filter(
        timestamps::Column::Ts.between(
            Local::now().naive_local() - Duration::days(1),
            Local::now().naive_local(),
        ),
    )
        .paginate(db, 15);
    let ts = paginator.fetch_page(page).await.unwrap_or(Vec::new());
    let response: Response<timestamps::Model> = Response::from(ts);
    HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(response)
}

/// POST: /api/timestamps/{timestamp}
#[rustfmt::skip]
#[post("/api/timestamps")]
pub async fn store_timestamp(db: web::Data<DB>, timestamp_data: web::Json<PostTimestamp>) -> Result<HttpResponse, Box<dyn std::error::Error>>{
    let db = &db.0;
    let mut timestamp = timestamp_data.into_inner();
    match Resident::find().filter(residents::Column::Rfid.eq(timestamp.rfid.clone())).one(db).await? {
        Some(resident) => {
             let mut resident = resident.into_active_model();
                if timestamp.location == resident.current_location.to_owned().unwrap() {
                    resident.current_location = Set(0);
                    timestamp.location = 0;
                } else {
                    resident.current_location = Set(timestamp.location);
                }

                let updated_resident = resident.save(db).await?;
                let new_timestamp: timestamps::ActiveModel = timestamps::ActiveModel {
                        rfid: Set(updated_resident.id.to_owned().unwrap()),
                        location: Set(updated_resident.current_location.to_owned().unwrap()),
                    ..Default::default()
                };
                let new_ts = new_timestamp.save(db).await?;

                let response = Response::<ResidentTimestamp>::from(ResidentTimestamp {
                    resident: updated_resident.try_into_model().unwrap(),
                    timestamp: new_ts.try_into_model().unwrap(),
                });
                Ok(HttpResponse::Ok().content_type(ContentType::json()).status(StatusCode::CREATED).json(response))
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

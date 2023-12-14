use super::timestamps::{PostTimestamp, ResidentTimestamp};
use actix_web::ResponseError;
use entity::locations;
use entity::prelude::OrmSerializable as Serializable;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

impl Serializable for PostTimestamp {}
impl Serializable for ResidentTimestamp {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<Vec<T>>,
}
impl From<entity::timestamps::Model> for PostTimestamp {
    fn from(value: entity::timestamps::Model) -> Self {
        Self {
            rfid: value.rfid,
            location: value.location as usize,
        }
    }
}
impl From<entity::timestamps::ActiveModel> for PostTimestamp {
    fn from(value: entity::timestamps::ActiveModel) -> Self {
        Self {
            rfid: value.rfid.into_value().unwrap().to_string(),
            location: value
                .location
                .into_value()
                .unwrap_or(0.into())
                .to_string()
                .parse::<usize>()
                .unwrap_or(0),
        }
    }
}
impl<T> From<PostTimestamp> for Response<T>
where
    T: From<PostTimestamp> + Serializable,
{
    fn from(value: PostTimestamp) -> Self {
        Self {
            success: true,
            message: "Successfully retrieved timestamp".to_string(),
            data: Some(vec![T::from(value)]),
        }
    }
}

impl<T: Serializable> Display for Response<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<T: Serializable + std::fmt::Debug> ResponseError for Response<T> {}

impl<T: Serializable> From<ResidentTimestamp> for Response<T>
where
    T: From<ResidentTimestamp>,
{
    fn from(res_ts: ResidentTimestamp) -> Self {
        Self {
            success: true,
            message: "Resident Timestamp successfully retrieved".to_string(),
            data: Some(vec![T::from(res_ts)]),
        }
    }
}
impl<T> From<locations::Model> for Response<T>
where
    T: From<locations::Model> + Serializable,
{
    fn from(value: locations::Model) -> Self {
        Self {
            success: true,
            message: "Location successfully retrieved".to_string(),
            data: Some(vec![T::from(value)]),
        }
    }
}

impl<T> From<Vec<entity::residents::Model>> for Response<T>
where
    T: From<entity::residents::Model> + Serializable,
    Vec<T>: From<Vec<entity::residents::Model>>,
{
    fn from(value: Vec<entity::residents::Model>) -> Self {
        Self {
            success: true,
            message: "Residents successfully retrieved".to_string(),
            data: Some(value.into()),
        }
    }
}
impl<T> From<Vec<entity::timestamps::Model>> for Response<T>
where
    T: From<entity::timestamps::Model> + Serializable,
    Vec<T>: From<Vec<entity::timestamps::Model>>,
{
    fn from(value: Vec<entity::timestamps::Model>) -> Self {
        Self {
            success: true,
            message: "Timestamps successfully retrieved".to_string(),
            data: Some(value.into()),
        }
    }
}

impl<T> From<Vec<entity::locations::Model>> for Response<T>
where
    T: From<entity::locations::Model> + Serializable,
    Vec<T>: From<Vec<entity::locations::Model>>,
{
    fn from(value: Vec<entity::locations::Model>) -> Self {
        Self {
            success: true,
            message: "Location successfully retrieved".to_string(),
            data: Some(value.into()),
        }
    }
}

impl<T> From<entity::timestamps::Model> for Response<T>
where
    T: From<entity::timestamps::Model> + Serializable,
{
    fn from(value: entity::timestamps::Model) -> Self {
        Self {
            success: true,
            message: "Timestamp successfully retrieved".to_string(),
            data: Some(vec![T::from(value)]),
        }
    }
}
impl<T> From<entity::residents::Model> for Response<T>
where
    T: From<entity::residents::Model> + Serializable,
{
    fn from(value: entity::residents::Model) -> Self {
        Self {
            success: true,
            message: "Residents successfully retrived".to_string(),
            data: Some(vec![T::from(value)]),
        }
    }
}

impl<T> Response<T>
where
    T: Serializable + std::fmt::Debug,
{
    pub fn from_success(msg: &str) -> Self {
        Self {
            success: true,
            message: msg.to_string(),
            data: None,
        }
    }
    pub fn from_error(msg: &str) -> Self {
        Self {
            success: false,
            message: msg.to_string(),
            data: None,
        }
    }
}

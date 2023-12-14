//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use crate::prelude::OrmSerializable;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

impl OrmSerializable for Model {}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "residents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub rfid: String,
    pub name: String,
    pub doc: String,
    pub room: String,
    pub unit: i32,
    pub current_location: i32,
    pub level: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::locations::Entity",
        from = "Column::CurrentLocation",
        to = "super::locations::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Locations2,
    #[sea_orm(
        belongs_to = "super::locations::Entity",
        from = "Column::Unit",
        to = "super::locations::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Locations1,
    #[sea_orm(has_many = "super::timestamps::Entity")]
    Timestamps,
}

impl Related<super::timestamps::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Timestamps.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

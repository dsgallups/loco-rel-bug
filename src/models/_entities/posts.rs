//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub content: String,
    pub contributors: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::authors::Entity",
        from = "Column::Contributors",
        to = "super::authors::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Authors,
}

impl Related<super::authors::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Authors.def()
    }
}

use std::fmt::Display;
use std::fmt;

use super::super::schema::*;
use diesel::prelude::*;

#[derive(Debug, diesel_derive_enum::DbEnum, Clone, Copy, PartialEq, Eq)]
#[DieselTypePath = "crate::schema::sql_types::GroupStatus"]
pub enum GroupStatus {
    Open,
    Closed,
}
impl Display for GroupStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        self::GroupStatus::Open => write!(f, "Open"),
        self::GroupStatus::Closed => write!(f, "Closed"),
      }
    }
}

#[derive(Queryable, Clone)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub status: GroupStatus,
}

#[derive(Insertable)]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
}

impl NewGroup {
    pub fn new(name: String) -> NewGroup {
        NewGroup { name }
    }
}
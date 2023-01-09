use super::super::schema::*;
use diesel::prelude::*;
use std::fmt::Display;
use std::fmt;


#[derive(Debug, diesel_derive_enum::DbEnum, PartialEq, Eq)]
#[DieselTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    User,
    Admin,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
        self::UserRole::User => write!(f, "User"),
        self::UserRole::Admin => write!(f, "Admin"),
      }
    }
}

#[derive(Queryable)]
pub struct GroupUser {
    pub group_id: i32,
    pub user_id: i32,
    pub role: UserRole,
    pub ward_id: Option<i32>,
}

#[derive(Insertable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "group_user"]
pub struct NewGroupUser {
    pub group_id: i32,
    pub user_id: i32,
    pub user_role: UserRole,
    pub ward_id: Option<i32>,
}

impl NewGroupUser {
  pub fn new(group_id: i32, user_id: i32, user_role: UserRole, ward_id: Option<i32>) -> NewGroupUser {
    NewGroupUser { group_id, user_id, user_role, ward_id }
  }
}
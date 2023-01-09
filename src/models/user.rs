use super::super::schema::*;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Clone, Identifiable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}
impl NewUser {
    pub fn new(name: String) -> NewUser {
        NewUser { name }
    }
}
use super::super::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Clone, Identifiable)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

impl NewUser {
    pub fn new(name: String) -> NewUser {
        NewUser { name }
    }
}
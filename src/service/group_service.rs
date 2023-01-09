use super::database_connection::Database;
use crate::models::group::Group;
use crate::models::group_user::*;

use crate::schema::groups::dsl::*;
use diesel::prelude::*;

pub struct GroupService {
    pub conn: diesel::PgConnection,
}

impl GroupService {
    pub fn new() -> GroupService {
        GroupService { conn: (Database::connect()) }
    }

    pub fn get_group_by_name(&mut self, groupname: &String) -> Option<Group> {
        let group = groups
            .filter(ExpressionMethods::eq(name, groupname))
            .first::<Group>(&mut self.conn);
        match group {
            Ok(u) => Some(u),
            Err(..) => None,
        }
    }

    pub fn count_admins(&mut self, group: &Group) -> i64 {
        use crate::schema::group_user::dsl::*;
        let admin_count = group_user
            .filter(BoolExpressionMethods::and(
                group_id.eq(group.id),
                user_role.eq(UserRole::Admin),
            ))
            .count()
            .get_result::<i64>(&mut self.conn); //count(group_user::group_id.eq(group.id));
        let admin_count = admin_count.unwrap();
        println!("Count admins in group {} is {}", group.name, admin_count);
        admin_count
    }
}
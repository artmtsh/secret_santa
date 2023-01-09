use crate::schema::group_user;
use crate::schema::groups;
use super::database_connection::Database;
use crate::models::group::*;
use crate::models::group_user::*;
use crate::models::user::*;
use crate::service::group_service::GroupService;
use diesel::prelude::*;
pub struct UserService {
    pub conn: diesel::PgConnection,
}
impl UserService {
    pub fn new() -> UserService {
        UserService {
            conn: (Database::connect()),
        }
    }
    pub fn create_user(&mut self, username: &String) -> Result<User, ()> {
        match self.get_user_by_name(username) {
            None => {}
            Some(..) => {
                println!("User with this name already exists!");
                return Err(());
            }
        }
        let new_user = NewUser::new(String::clone(username));
        use crate::schema::users::dsl::*;
        let user = diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&mut self.conn);
        match user {
            Ok(u) => {
                println!("Created user with id: {}, name: {}", u.id, u.name);
                Ok(u)
            }
            Err(..) => {
                println!("Error occured putting user {} in database", username);
                Err(())
            }
        }
    }
    fn get_user_by_name(&mut self, username: &String) -> Option<User> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(ExpressionMethods::eq(name, username))
            .first::<User>(&mut self.conn);
        match user {
            Ok(u) => Some(u),
            Err(..) => None,
        }
    }
    pub fn create_group(&mut self, caller_name: &String, group_name: &String) -> Result<Group, ()> {
        if !Self::is_user_in_database(&caller_name) {
            return Err(());
        }
        match GroupService::new().get_group_by_name(group_name) {
            None => {}
            Some(..) => {
                println!("Group with this name already exists!");
                return Err(());
            }
        }
        let caller = self.get_user_by_name(caller_name).unwrap();
        let new_group = NewGroup::new(String::clone(&group_name));
        let group = diesel::insert_into(groups::table)
            .values(&new_group)
            .get_result::<Group>(&mut self.conn);
        let group = group.unwrap();
        println!(
            "Created group with id: {}, name: {}, status: {}",
            group.id, group.name, group.status
        );
        Self::set_first_admin(&caller, &group, &mut self.conn);
        Ok(group)
    }
    fn set_first_admin(user: &User, group: &Group, conn: &mut PgConnection) {
        Self::create_group_user_link(group, user, UserRole::Admin, conn);
    }
    fn create_group_user_link(
        group: &Group,
        new_member: &User,
        role: UserRole,
        conn: &mut PgConnection,
    ) -> Result<(), ()> {
        let new_group_user = NewGroupUser::new(group.id, new_member.id, role, None);
        if Self::is_user_in_group(&new_member, &group, conn) {
            println!("User {} is already in {}", new_member.name, group.name);
            return Err(());
        }
        let group_user = diesel::insert_into(group_user::table)
            .values(&new_group_user)
            .get_result::<GroupUser>(conn);
        let group_user = group_user.unwrap();
        println!(
            "Created group_user link: group: {}, user: {}, role: {}",
            group.name, new_member.name, group_user.role
        );
        Ok(())
    }
    fn is_user_in_database(user_name: &String) -> bool {
        match UserService::new().get_user_by_name(&user_name) {
            None => {
                println!("User with name {} was not found", user_name);
                false
            }
            Some(..) => true,
        }
    }
    fn is_user_in_group(user: &User, group: &Group, conn: &mut PgConnection) -> bool {
        use crate::schema::group_user::dsl::*;
        let user = group_user
            .filter(BoolExpressionMethods::and(
                ExpressionMethods::eq(group_id, group.id),
                ExpressionMethods::eq(user_id, user.id),
            ))
            .first::<GroupUser>(conn);
        match user {
            Ok(..) => true,
            Err(..) => false,
        }
    }
    pub fn join_group(&mut self, caller_name: &String, group_name: &String) -> Result<(), ()> {
        if !Self::is_user_in_database(&caller_name) {
            return Err(());
        }
        let mut caller = self.get_user_by_name(caller_name).unwrap();
        let mut group_service = GroupService::new();
        let group = match group_service.get_group_by_name(&group_name) {
            None => {
                println!("Group with this name does not exist!");
                return Err(());
            }
            Some(g) => g,
        };
        if group.status == GroupStatus::Closed {
            println!("Group with name {} is closed", group.name);
            return Err(());
        }
        Self::create_group_user_link(&group, &caller, UserRole::User, &mut self.conn);
        Ok(())
    }
    fn is_group_in_database(group_name: &String) -> bool {
        match GroupService::new().get_group_by_name(&group_name) {
            None => false,
            Some(..) => true,
        }
    }
    pub fn close_group(&mut self, caller: &User, group: &Group) -> Result<(), ()> {
        use crate::schema::groups::dsl::*;
        if !Self::is_user_in_database(&caller.name) {
            println!("User {} was not found in database", caller.name);
            return Err(());
        }
        if !Self::is_group_in_database(&group.name) {
            println!("Group {} was not found in database", group.name);
            return Err(());
        }
        if !Self::is_user_in_group(caller, group, &mut self.conn) {
            println!("User {} was not found in group {}", caller.name, group.name);
            return Err(());
        }
        if !Self::is_admin(caller, group, &mut self.conn) {
            println!("User {} is not admin in group {}", caller.name, group.name);
            return Err(());
        }
        diesel::update(groups.filter(id.eq(group.id)))
            .set(current_state.eq(GroupStatus::Closed))
            .get_result::<Group>(&mut self.conn);
        Ok(())
    }
    fn is_admin(user: &User, group: &Group, conn: &mut PgConnection) -> bool {
        use crate::schema::group_user::dsl::*;
        let user = group_user
            .filter(BoolExpressionMethods::and(
                BoolExpressionMethods::and(group_id.eq(group.id), user_id.eq(user.id)),
                user_role.eq(UserRole::Admin),
            ))
            .first::<GroupUser>(conn);
        match user {
            Ok(..) => true,
            Err(..) => false,
        }
    }
    pub fn set_admin(
        &mut self,
        caller_name: &String,
        new_admin_name: &String,
        group_name: &String,
    ) -> Result<(), ()> {
        if !Self::is_user_in_database(&caller_name) {
            println!("User {} was not found in database", caller_name);
            return Err(());
        }
        if !Self::is_user_in_database(&new_admin_name) {
            println!("User {} was not found in database", new_admin_name);
            return Err(());
        }
        let caller = self.get_user_by_name(caller_name).unwrap();
        if !Self::is_group_in_database(&group_name) {
            println!("Group {} was not found in database", group_name);
            return Err(());
        }
        let new_admin = UserService::new().get_user_by_name(new_admin_name).unwrap();
        let group = GroupService::new().get_group_by_name(group_name).unwrap();
        if !Self::is_user_in_group(&caller, &group, &mut self.conn) {
            println!("User {} was not found in group {}", caller.name, group_name);
            return Err(());
        }
        if !Self::is_user_in_group(&new_admin, &group, &mut self.conn) {
            println!(
                "User {} was not found in group {}",
                new_admin.name, group_name
            );
            return Err(());
        }
        if !Self::is_admin(&caller, &group, &mut self.conn) {
            println!("User {} is not admin in group {}", caller.name, group.name);
            return Err(());
        }
        use crate::schema::group_user::dsl::*;
        let res = diesel::update(group_user.filter(BoolExpressionMethods::and(
            user_id.eq(new_admin.id),
            group_id.eq(group.id),
        )))
        .set(user_role.eq(UserRole::Admin))
        .get_result::<GroupUser>(&mut self.conn);
        match res {
            Ok(..) => {
                println!(
                    "User {} made user {} admin in group {}",
                    caller.name, new_admin.name, group_name
                );
                Ok(())
            }
            Err(..) => {
                println!(
                    "Error occured while making user {} new admin",
                    new_admin_name
                );
                Err(())
            }
        }
    }
    pub fn retire(&mut self, caller_name: &String, group_name: &String) -> Result<(), ()> {
        if !Self::is_user_in_database(&caller_name) {
            println!("User {} was not found in database", caller_name);
            return Err(());
        }
        if !Self::is_group_in_database(&group_name) {
            println!("Group {} was not found in database", group_name);
            return Err(());
        }
        let caller = self.get_user_by_name(caller_name).unwrap();
        let group = GroupService::new().get_group_by_name(group_name).unwrap();
        if !Self::is_user_in_group(&caller, &group, &mut self.conn) {
            println!("User {} was not found in group {}", caller.name, group.name);
            return Err(());
        }
        if !Self::is_admin(&caller, &group, &mut self.conn) {
            println!(
                "User {} already is not admin in group {}",
                caller.name, group.name
            );
            return Err(());
        }
        if GroupService::new().count_admins(&group) <= 1 {
            println!(
                "User {} can not retire from the group {} due to litle number of admins",
                caller.name, group_name
            );
            return Err(());
        }
        use crate::schema::group_user::dsl::*;
        diesel::update(group_user.filter(BoolExpressionMethods::and(
            user_id.eq(caller.id),
            group_id.eq(group.id),
        )))
        .set(user_role.eq(UserRole::User))
        .get_result::<GroupUser>(&mut self.conn);
        println!("User {} was retired in group {}", caller.name, group_name);
        Ok(())
    }
    pub fn leave(&mut self, caller_name: &String, group_name: &String) -> Result<(), ()> {
        if !Self::is_user_in_database(&caller_name) {
            println!("User {} was not found in database", caller_name);
            return Err(());
        }
        if !Self::is_group_in_database(&group_name) {
            println!("Group {} was not found in database", group_name);
            return Err(());
        }
        let caller = self.get_user_by_name(caller_name).unwrap();
        let group = GroupService::new().get_group_by_name(group_name).unwrap();
        if !Self::is_user_in_group(&caller, &group, &mut self.conn) {
            println!("User {} was not found in group {}", caller.name, group.name);
            return Err(());
        }
        if Self::is_admin(&caller, &group, &mut self.conn)
            && GroupService::new().count_admins(&group) <= 1
        {
            println!(
                "User {} can not leave from the group {} due to litle number of admins",
                caller.name, group_name
            );
            return Err(());
        }
        if group.status == GroupStatus::Closed {
            println!(
                "User {} can not leave from the group {} because it is closed",
                caller.name, group_name
            );
            return Err(());
        }
        use crate::schema::group_user::dsl::*;
        diesel::dsl::delete(group_user.filter(BoolExpressionMethods::and(
            user_id.eq(caller.id),
            group_id.eq(group.id),
        )))
        .get_result::<GroupUser>(&mut self.conn);
        println!("User {} left the group {}", caller.name, group_name);
        Ok(())
    }
    pub fn delete_group(&mut self, caller_name: &String, group_name: &String) -> Result<(), ()> {
        if !Self::is_user_in_database(&caller_name) {
            println!("User {} was not found in database", caller_name);
            return Err(());
        }
        if !Self::is_group_in_database(&group_name) {
            println!("Group {} was not found in database", group_name);
            return Err(());
        }
        let caller = self.get_user_by_name(caller_name).unwrap();
        let group = GroupService::new().get_group_by_name(group_name).unwrap();
        if !Self::is_user_in_group(&caller, &group, &mut self.conn) {
            println!("User {} was not found in group {}", caller.name, group.name);
            return Err(());
        }
        if !Self::is_admin(&caller, &group, &mut self.conn) {
            println!(
                "User {} can not delete the group {} due to no admin rights",
                caller.name, group_name
            );
            return Err(());
        }
        use crate::schema::groups::dsl::*;
        diesel::dsl::delete(groups.filter(id.eq(group.id))).get_result::<Group>(&mut self.conn);
        println!("User {} deleted the group {}", caller.name, group_name);
        Ok(())
    }
    pub fn start_secret_santa(
        &mut self,
        caller_name: &String,
        group_name: &String,
    ) -> Result<(), ()> {
        if !Self::is_user_in_database(&caller_name) {
            println!("User {} was not found in database", caller_name);
            return Err(());
        }
        if !Self::is_group_in_database(&group_name) {
            println!("Group {} was not found in database", group_name);
            return Err(());
        }
        let caller = self.get_user_by_name(caller_name).unwrap();
        let group = GroupService::new().get_group_by_name(group_name).unwrap();
        if !Self::is_user_in_group(&caller, &group, &mut self.conn) {
            println!("User {} was not found in group {}", caller_name, group.name);
            return Err(());
        }
        if !Self::is_admin(&caller, &group, &mut self.conn) {
            println!(
                "User {} can not delete the group {} due to no admin rights",
                caller.name, group_name
            );
            return Err(());
        }
        if group.status == GroupStatus::Closed {
            println!(
                "User {} can not start the game because the group {} because it is closed",
                caller.name, group_name
            );
            return Err(());
        }
        use crate::schema::group_user::dsl::*;
        let mut user_id_vec = group_user
            .filter(group_id.eq(group.id))
            .select(user_id)
            .load::<i32>(&mut self.conn)
            .unwrap();
        if user_id_vec.len() <= 2 {
            println!(
                "Can not start the game for the group {} due to too little number of participants",
                group_name
            );
            return Err(());
        }
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        user_id_vec.shuffle(&mut thread_rng());
        let mut shifted_user_id_vec = user_id_vec.clone();
        Self::shift_left(&mut shifted_user_id_vec);
        for i in 0..user_id_vec.len() {
            let res = diesel::dsl::update(group_user.filter(BoolExpressionMethods::and(
                user_id.eq(user_id_vec[i]),
                group_id.eq(group.id),
            )))
            .set(ward_id.eq(shifted_user_id_vec[i]))
            .get_result::<GroupUser>(&mut self.conn);
            match res {
                Ok(..) => {}
                Err(..) => {
                    println!("Error occured trying to set ward. Please start the game again.");
                    return Err(());
                }
            }
        }
        println!(
            "Game for group {} has been started successfully by user {}",
            group_name, caller.name
        );
        println!("{:?}", user_id_vec);
        println!("{:?}", shifted_user_id_vec);
        self.close_group(&caller, &group);
        Ok(())
    }
    fn shift_left(arr: &mut Vec<i32>) {
        let first = arr.first().unwrap().clone();
        for i in 1..arr.len() {
            arr[i - 1] = arr[i].clone();
        }
        let len = arr.len();
        arr[len - 1] = first;
    }
}
// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "group_status"))]
    pub struct GroupStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    group_user (group_id, user_id) {
        group_id -> Int4,
        user_id -> Int4,
        user_role -> UserRole,
        ward_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GroupStatus;

    groups (id) {
        id -> Int4,
        name -> Varchar,
        current_state -> GroupStatus,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(group_user -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_user,
    groups,
    users,
);
use diesel::QueryableByName;
use diesel_derive_enum::DbEnum;
use chrono::NaiveDateTime;

use crate::schema::users;

#[derive(Debug, DbEnum)]
pub enum Role {
    Regular, Admin, Guest
}

#[derive(QueryableByName, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub password_hash: String,
    pub avatar_img_id: Option<String>,
    pub login: String,
    pub name: String,
    pub role: Role,
    pub last_update_date: NaiveDateTime,
    pub creation_date: NaiveDateTime
}

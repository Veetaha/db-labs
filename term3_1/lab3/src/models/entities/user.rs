use chrono::NaiveDateTime;
use diesel::{Queryable, Identifiable};
use crate::{models::schema::users, cli};

pub use cli::{UserRole, UserRoleMapping};


#[derive(Debug, Queryable, Identifiable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub password_hash: String,
    pub avatar_img_id: Option<String>,
    pub login: String,
    pub name: String,
    pub role: UserRole,
    pub last_update_date: NaiveDateTime,
    pub creation_date: NaiveDateTime
}

#[derive(Debug, AsChangeset, Identifiable)]
#[table_name="users"]
pub struct UserUpdate {
    pub id: i32,
    pub avatar_img_id: Option<Option<String>>,
    pub login: Option<String>,
    pub name: Option<String>,
    pub role: Option<UserRole>
}

impl From<crate::cli::UserUpdate> for UserUpdate {

    fn from(cli_upd: crate::cli::UserUpdate) -> Self { Self {
        id:            cli_upd.id,
        avatar_img_id: cli_upd.data.avatar_img_id,
        login:         cli_upd.data.login,
        name:          cli_upd.data.name,
        role:          cli_upd.data.role
    }}

}

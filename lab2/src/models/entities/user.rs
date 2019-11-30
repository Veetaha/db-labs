use chrono::NaiveDateTime;
use pg::row::Row as PgRow;
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
#[postgres(name="userrole")]
pub enum UserRole {
    Regular, Admin, Guest
}

#[derive(Debug)]
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

impl From<PgRow> for User {
    fn from(row: PgRow) -> Self { Self {
        id:               row.get("id"),
        password_hash:    row.get("password_hash"),
        avatar_img_id:    row.get("avatar_img_id"),
        login:            row.get("login"),
        name:             row.get("name"),
        role:             row.get("role"),
        last_update_date: row.get("last_update_date"),
        creation_date:    row.get("creation_date"),
    }}
}

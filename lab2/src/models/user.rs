use diesel::Queryable;

pub enum Role {
    Regular, Admin, Guest
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub password_hash: String,
    pub avatar_img_id: Option<String>,
    pub login: String,
    pub name: String,
    pub role: Role,
    pub last_update_date: String, // timestamp
    pub creation_date: String
}

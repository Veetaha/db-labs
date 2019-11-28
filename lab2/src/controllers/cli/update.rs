use structopt::{StructOpt};

use super::enums::UserRole;

#[derive(StructOpt, Debug)]
pub enum Update {
    User(UserUpdate)
}

#[derive(StructOpt, Debug)]
pub struct UserUpdate {
    /// Unique identifier of the entity to update.
    pub id: i32,

    /// Set new avatar image id (remove avatar if value is omitted)
    #[structopt(long)]
    pub avatar_img_id: Option<Option<String>>,

    /// Set new user login
    #[structopt(long)]
    pub login: Option<String>,

    /// Set new user name
    #[structopt(long)]
    pub name: Option<String>,

    /// Set new user role
    #[structopt(
        long,
        case_insensitive = true,
        possible_values = &UserRole::variants()
    )]
    pub role: Option<UserRole>
}

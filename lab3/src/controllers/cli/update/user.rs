use structopt::{StructOpt, clap::ArgGroup};

use crate::cli::enums::UserRole;

#[derive(StructOpt, Debug)]
pub struct UserUpdate {
    /// Unique identifier of the entity to update.
    pub id: i32,

    #[structopt(flatten)]
    pub data: UserUpdateData
}

#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("user_upd").required(true).multiple(true))]
pub struct UserUpdateData {
    /// Set new avatar image id (remove avatar if value is omitted)
    #[structopt(long, group = "user_upd")]
    pub avatar_img_id: Option<Option<String>>,

    /// Set new user login
    #[structopt(long, group = "user_upd")]
    pub login: Option<String>,

    /// Set new user name
    #[structopt(long, group = "user_upd")]
    pub name: Option<String>,

    /// Set new user role
    #[structopt(
        long,
        group = "user_upd", 
        case_insensitive = true,
        possible_values = &UserRole::variants()
    )]
    pub role: Option<UserRole>
}


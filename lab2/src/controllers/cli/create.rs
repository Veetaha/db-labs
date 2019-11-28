use structopt::StructOpt;

use super::enums::UserRole;

#[derive(StructOpt, Debug)]
pub enum Create {
    User(CreateUser)
}

#[derive(StructOpt, Debug)]
pub struct CreateUser {
    /// User's avatar image id (absent by default).
    #[structopt(long)]
    pub avatar_img_id: Option<String>,

    /// User's login (must be globally unique)
    #[structopt(long)]
    pub login: String,

    /// User name or nickname
    #[structopt(long)]
    pub name: String,

    /// User's password
    pub password: String,

    /// User role
    #[structopt(
        long,
        possible_values = &UserRole::variants(),
        case_insensitive = true,
        default_value = "regular"
    )]
    pub role: UserRole
}

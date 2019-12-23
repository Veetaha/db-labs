use structopt::StructOpt;
use diesel::Insertable;

use crate::{ cli::UserRole, models::schema::users };

#[derive(StructOpt, Debug, Insertable)]
#[table_name="users"]
pub struct UserNew {
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
    #[structopt(long)]
    #[column_name="password_hash"]
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

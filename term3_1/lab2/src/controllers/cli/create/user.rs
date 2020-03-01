use structopt::StructOpt;

use crate::{
    cli::UserRole,
    models::traits::{ColData, ColDataVec}
};

#[derive(StructOpt, Debug)]
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


impl From<UserNew> for ColDataVec {

    fn from(new_user: UserNew) -> Self { 
        let password_hash = bcrypt::hash(new_user.password, bcrypt::DEFAULT_COST).expect(
            "Some precondition of password hash algorithm was violated"
        );

        let mut cols = Self::new();

        cols.push(ColData::with_boxed("password_hash", password_hash));
        cols.push(ColData::with_boxed("avatar_img_id", new_user.avatar_img_id));
        cols.push(ColData::with_boxed("login",         new_user.login));
        cols.push(ColData::with_boxed("name",          new_user.name));
        cols.push(ColData::with_boxed("role",          new_user.role));

        cols
    }

}


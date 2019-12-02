use structopt::StructOpt;

use crate::{
    cli::enums::UserRole,
    models::traits::{ColData, ColDataVec, GetId}
};

#[derive(StructOpt, Debug)]
pub struct UserUpdate {
    /// Unique identifier of the entity to update.
    pub id: i32,

    #[structopt(flatten)]
    pub data: UserUpdateData
}

// TODO: add ArgGroup once this bug is fixed: https://github.com/TeXitoi/structopt/issues/151
#[derive(StructOpt, Debug)]
pub struct UserUpdateData {
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

impl GetId for UserUpdate {
    fn get_id(&self) -> i32 { self.id }
}

impl From<UserUpdate> for ColDataVec {

    fn from(UserUpdate { data, .. }: UserUpdate) -> Self {
        let mut cols = Self::new();

        ColData::try_push(&mut cols, "avatar_img_id", data.avatar_img_id);
        ColData::try_push(&mut cols, "login",         data.login);
        ColData::try_push(&mut cols, "name",          data.name);
        ColData::try_push(&mut cols, "role",          data.role);
        
        return cols;
    }
}

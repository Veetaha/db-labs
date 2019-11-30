use structopt::{StructOpt, clap::ArgGroup};

use crate::models::traits::{ColData, ColDataVec, GetId};
use super::enums::UserRole;

#[derive(StructOpt, Debug)]
pub enum Update {
    User(UserUpdate)
}

#[derive(StructOpt, Debug)]
pub struct UserUpdate {
    /// Unique identifier of the entity to update.
    pub id: i32,

    #[structopt(flatten)]
    pub data: UserUpdateData
}

#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("user_update_data").required(true))]
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
        use pg::types::ToSql;

        fn try_push<T: 'static + ToSql>(col_upds: &mut ColDataVec, col: &str, val: Option<T>) {
            if let Some(val) = val {
                col_upds.push(ColData::with_boxed(col, val));
            }
        }

        let mut result = Self::new();

        try_push(&mut result, "avatar_img_id", data.avatar_img_id);
        try_push(&mut result, "login", data.login);
        try_push(&mut result, "name", data.name);
        try_push(&mut result, "role", data.role);
        
        return result;
    }
}

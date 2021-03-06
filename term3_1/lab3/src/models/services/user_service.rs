use anyhow::{Result, Context};
use diesel::prelude::*;
use crate::{models::entities::{self, User}, cli, database::PgConnPool};

pub struct UserService {
    pg_conn_pool: PgConnPool
}

use crate::models::schema::users::dsl as DieselUserDsl;

impl UserService {

    pub fn new(pg_conn_pool: PgConnPool) -> Self {
        Self { pg_conn_pool }
    }

    pub fn create(&self, user_new: cli::UserNew) -> Result<User> {
        use DieselUserDsl::*;

        diesel::insert_into(users)
            .values(&user_new)
            .get_result(&*self.pg_conn_pool.get().unwrap())
            .context("Failed to create user")
    }

    pub fn get_by_id(&self, target_id: i32) -> Result<User> {
        use DieselUserDsl::*;

        users.filter(id.eq(target_id))
            .get_result(&*self.pg_conn_pool.get().unwrap())
            .context("Failed to get user by id")
    }

    pub fn update_by_id(&self, upd: cli::UserUpdate) -> Result<User> {
        entities::UserUpdate::from(upd)
            .save_changes(&*self.pg_conn_pool.get().unwrap())
            .context("Failed to update user by id")
    }

    pub fn delete_by_id(&self, target_id: i32) -> Result<bool> {
        use DieselUserDsl::*;

        diesel::delete(users).filter(id.eq(target_id))
            .execute(&*self.pg_conn_pool.get().unwrap())
            .map(|rows_affected| rows_affected > 0)
            .context("Failed to delte user by id")
    }
}

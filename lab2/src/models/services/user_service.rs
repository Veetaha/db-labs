use diesel::QueryResult;
use crate::{
    models::{
        PgConnPool,
        entities::User,
        traits
    },
    cli::{ CreateUser, UserUpdate }
};

pub struct UserService {
    pg_conn_pool: PgConnPool
}

impl UserService {
    pub fn new(pg_conn_pool: PgConnPool) -> Self {
        Self { pg_conn_pool }
    }
    pub fn update(&self, user_update: &UserUpdate) -> QueryResult<User> {
        unimplemented!();
    }
    pub fn create(&self, new_user: &CreateUser) -> QueryResult<User> {
        unimplemented!();
    }
}

impl traits::EntityService for UserService { type Entity = User; }
impl traits::GetTableName  for UserService { fn get_table_name(&self) -> &str { "users" } }
impl traits::GetPgConnPool for UserService { fn get_pg_conn_pool(&self) -> PgConnPool { self.pg_conn_pool.clone() } }
impl traits::GetPgConnFromPoolInfallible for UserService {}

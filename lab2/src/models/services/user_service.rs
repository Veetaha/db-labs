use crate::{
    models::{
        entities::User,
        traits
    },
    cli::{ UserNew, UserUpdate },
    database::PgConnPool
};

pub struct UserService {
    pg_conn_pool: PgConnPool
}

impl UserService {
    pub fn new(pg_conn_pool: PgConnPool) -> Self {
        Self { pg_conn_pool }
    }
}

impl traits::EntityService          for UserService { type Entity = User; }
impl traits::UpdatableEntityService for UserService { type EntityUpd = UserUpdate; }
impl traits::CreatableEntityService for UserService { type EntityNew = UserNew; }
impl traits::GetTableName  for UserService { fn get_table_name(&self) -> &str { "users" } }
impl traits::GetPgConnPool for UserService {
    fn get_pg_conn_pool(&self) -> PgConnPool {
        // Clone operation is fast and efficient since it is just an atomic reference counter
        // under the hood, thus to make it explicit calling `clone()`
        // not as a metod but via free function call syntax to make it explicit
        PgConnPool::clone(&self.pg_conn_pool)
    }
}
impl traits::GetPgClientFromPoolInfallible for UserService {}


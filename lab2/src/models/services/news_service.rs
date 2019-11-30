// use crate::models::{
//     PgConnPool,
//     entities::News,
//     traits::{GetTableName, GetPgConnPool, GetPgConnFromPoolInfallible, GetById}
// };

// pub struct NewsService {
//     pg_conn_pool: PgConnPool
// }

// impl NewsService {
//     pub fn new(pg_conn_pool: PgConnPool) -> Self {
//         Self { pg_conn_pool }
//     }
// }

// impl GetTableName for NewsService { fn get_table_name(&self) -> &str { "news" } }
// impl GetPgConnPool for NewsService { fn get_pg_conn_pool(&self) -> PgConnPool { self.pg_conn_pool.clone() } }
// impl GetPgConnFromPoolInfallible for NewsService {}


use crate::models::{
    PgConnPool,
    entities::NewsComment,
    traits::{GetTableName, GetPgConnPool, GetPgConnFromPoolInfallible, GetById}
};

pub struct NewsCommentService {
    pg_conn_pool: PgConnPool
}

impl NewsCommentService {
    pub fn new(pg_conn_pool: PgConnPool) -> Self {
        Self { pg_conn_pool }
    }
}

impl GetTableName for NewsCommentService { fn get_table_name(&self) -> &str { "news_comments" } }
impl GetPgConnPool for NewsCommentService { fn get_pg_conn_pool(&self) -> PgConnPool { self.pg_conn_pool.clone() } }
impl GetPgConnFromPoolInfallible for NewsCommentService {}



use crate::{
    models::{
        entities::NewsRating,
        traits
    },
    cli::{ NewsRatingNew },
    database::PgConnPool
};

pub struct NewsRatingService {
    pg_conn_pool: PgConnPool
}

impl NewsRatingService {
    pub fn new(pg_conn_pool: PgConnPool) -> Self {
        Self { pg_conn_pool }
    }
}

impl traits::EntityService          for NewsRatingService { type Entity = NewsRating; }
impl traits::CreatableEntityService for NewsRatingService { type EntityNew = NewsRatingNew; }
impl traits::GetTableName  for NewsRatingService { fn get_table_name(&self) -> &str { "news_ratings" } }
impl traits::GetPgConnPool for NewsRatingService {
    fn get_pg_conn_pool(&self) -> PgConnPool {
        PgConnPool::clone(&self.pg_conn_pool)
    }
}
impl traits::GetPgClientFromPoolInfallible for NewsRatingService {}

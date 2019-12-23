use diesel::prelude::*;
use anyhow::{Result, Context};

use crate::cli;
use crate::models::schema::news_ratings::dsl as DieselNewsRatingDls;
use crate::models::entities::{NewsRating, NewsRatingNew};
use crate::database::PgConnPool;

pub struct NewsRatingService {
    pg_conn_pool: PgConnPool
}


impl NewsRatingService {
    pub fn new(pg_conn_pool: PgConnPool) -> Self {
        Self { pg_conn_pool }
    }

    pub fn create(&self, rating_new: cli::NewsRatingNew) -> Result<NewsRating> {
        use DieselNewsRatingDls::*;

        diesel::insert_into(news_ratings)
            .values(&NewsRatingNew::from(rating_new))
            .get_result(&*self.pg_conn_pool.get().unwrap())
            .context("Failed to create news rating")
    }

    pub fn get_by_id(&self, target_id: i32) -> Result<NewsRating> {
        use DieselNewsRatingDls::*;

        news_ratings.filter(id.eq(target_id))
            .get_result(&*self.pg_conn_pool.get().unwrap())
            .context("Failed to get news rating by id")
    }

}

// impl traits::EntityService          for NewsRatingService { type Entity = NewsRating; }
// impl traits::CreatableEntityService for NewsRatingService { type EntityNew = NewsRatingNew; }
// impl traits::GetTableName  for NewsRatingService { fn get_table_name(&self) -> &str { "news_ratings" } }
// impl traits::GetPgConnPool for NewsRatingService {
//     fn get_pg_conn_pool(&self) -> PgConnPool {
//         PgConnPool::clone(&self.pg_conn_pool)
//     }
// }
// impl traits::GetPgClientFromPoolInfallible for NewsRatingService {}

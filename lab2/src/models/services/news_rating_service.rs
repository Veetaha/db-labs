// use crate::models::{
//     PgConnPool,
//     entities::{NewsRating},
//     traits::{GetTableName, GetPgConnPool, GetPgConnFromPoolInfallible, GetById}
// };

// pub struct NewsRatingService {
//     pg_conn_pool: PgConnPool
// }

// impl NewsRatingService {
//     pub fn new(pg_conn_pool: PgConnPool) -> Self {
//         Self { pg_conn_pool }
//     }
// }

// impl GetTableName for NewsRatingService { fn get_table_name(&self) -> &str { "news_ratings" } }
// impl GetPgConnPool for NewsRatingService { fn get_pg_conn_pool(&self) -> PgConnPool { self.pg_conn_pool.clone() } }
// impl GetPgConnFromPoolInfallible for NewsRatingService {}

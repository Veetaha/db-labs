use anyhow::{Result, Context};
use crate::{
    models::traits,
    cli::PushRandomEntities,
    database::PgConnPool
};

pub struct GlobalDbService {
    pg_conn_pool: PgConnPool
}

impl GlobalDbService {
    pub fn new(pg_conn_pool: PgConnPool) -> Self {
        Self { pg_conn_pool }
    }

    pub fn push_random_entities(&self, opts: PushRandomEntities) -> Result<i32> {
        use traits::*;
        use std::convert::TryInto;
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let (min, max) = if opts.min_amount < opts.max_amount { 
            (opts.min_amount, opts.max_amount)
        } else { 
            (opts.max_amount, opts.min_amount)
        };

        let query = include_str!("./sql/generate_random_users.sql");


        let total: i32 = rng.gen_range(min, max)
            .try_into()
            .expect("specified value of max entites cannot fit in i32");

        self.get_pg_client()
            .execute(query, &[&total])
            .context("Failed to generate random users")?;

        Ok(total)
    }
}

impl traits::GetPgConnPool for GlobalDbService {
    fn get_pg_conn_pool(&self) -> PgConnPool {
        PgConnPool::clone(&self.pg_conn_pool)
    }
}
impl traits::GetPgClientFromPoolInfallible for GlobalDbService {}

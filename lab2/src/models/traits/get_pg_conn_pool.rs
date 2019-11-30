use crate::database::{PgConnPool, PooledPgClient};
use super::GetPgClient;

pub trait GetPgConnPool {
    fn get_pg_conn_pool(&self) -> PgConnPool;
}

/**
 * Provides implementation for retrieving database connection from connection pool
 * that aborts the workflow when fails.
 */
pub trait GetPgClientFromPoolInfallible: GetPgConnPool {}
impl <T: GetPgClientFromPoolInfallible> GetPgClient for T {

    type PgClientLike = PooledPgClient;

    fn get_pg_client(&self) -> Self::PgClientLike {
        self.get_pg_conn_pool()
            .get()
            .expect("Failed to retrieve database connection from pool")
    }
}

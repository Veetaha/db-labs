
use crate::models::{PgConnPool, PooledPgConn};
use super::GetPgConn;

pub trait GetPgConnPool {
    fn get_pg_conn_pool(&self) -> PgConnPool;
}

/**
 * Provides implementation for retrieving database connection from connection pool
 * that aborts the workflow when fails.
 */
pub trait GetPgConnFromPoolInfallible: GetPgConnPool {}
impl <T: GetPgConnFromPoolInfallible> GetPgConn for T {

    type PgConn = PooledPgConn;

    fn get_pg_conn(&self) -> Self::PgConn {
        self.get_pg_conn_pool()
            .get()
            .expect("Failed to retrieve connection from pool")
    }
}


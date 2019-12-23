use diesel::{PgConnection, r2d2::{Pool, PooledConnection, ConnectionManager}};

pub type PgConnManager = ConnectionManager<PgConnection>;
pub type PgConnPool    = Pool<PgConnManager>;
pub type PooledPgConn  = PooledConnection<PgConnManager>;

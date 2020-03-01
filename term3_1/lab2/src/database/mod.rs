mod pg_conn_mgr;
mod pg_placeholders_seq;
mod sql_params;

pub use pg_conn_mgr::*;
pub use pg_placeholders_seq::*;
pub use sql_params::*;

use pg::tls::NoTls;

pub type PgConnPool = r2d2::Pool<PgConnMgr<NoTls>>;
pub type PooledPgClient = r2d2::PooledConnection<PgConnMgr<NoTls>>;

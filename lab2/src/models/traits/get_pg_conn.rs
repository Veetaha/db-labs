use std::ops::Deref;
use diesel::{PgConnection};


pub trait GetPgConn {

    type PgConn: Deref<Target = PgConnection>;

    fn get_pg_conn(&self) -> Self::PgConn;
}

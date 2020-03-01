use std::ops::DerefMut;

pub trait GetPgConn {
    type DerefMutPgConn: DerefMut<Target = diesel::PgConnection>;

    fn get_pg_conn(&self) -> Self::DerefMutPgConn;
}

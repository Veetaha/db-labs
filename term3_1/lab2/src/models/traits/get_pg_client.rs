use std::ops::DerefMut;

pub trait GetPgClient {
    type PgClientLike: DerefMut<Target = pg::Client>;

    fn get_pg_client(&self) -> Self::PgClientLike;
}

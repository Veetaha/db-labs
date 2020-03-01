use anyhow::{Result, Context};
use diesel::{associations::HasTable, query_builder::IntoUpdateTarget};
use super::{GetPgConn};

pub trait DeleteById {
    fn delete_by_id(&self, id: i32) -> Result<bool>;
}

impl<T> DeleteById for T
where
    Self: HasTable + GetPgConn,
    Self::Table: IntoUpdateTarget
{
    fn delete_by_id(&self, id: i32) -> Result<bool> {
        diesel::delete(self.table())
            .execute(self.get_pg_conn())
            .context("delete operation failed")
            .map(|rows_affected| rows_affected > 0)
    }
}

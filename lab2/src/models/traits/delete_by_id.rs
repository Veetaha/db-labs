use anyhow::{Result, Context};
use diesel::prelude::*;

use super::{GetTableName, GetPgConn};

pub trait DeleteById {
    fn delete_by_id(&self, id: i32) -> Result<bool>;
}

impl<T> DeleteById for T
where
    Self: GetTableName + GetPgConn
{
    fn delete_by_id(&self, id: i32) -> Result<bool> {
        use diesel::sql_types::*;

        diesel::sql_query(format!("delete from {} where id = $1;", self.get_table_name()))
            .bind::<Integer, _>(id)
            .execute(&*self.get_pg_conn())
            .context("delete operation failed")
            .map(|rows_affected| rows_affected > 0)
    }
}

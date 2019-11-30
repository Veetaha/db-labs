use anyhow::{Result, Context};

use super::{GetTableName, GetPgClient};

pub trait DeleteById {
    fn delete_by_id(&self, id: i32) -> Result<bool>;
}

impl<T> DeleteById for T
where
    Self: GetTableName + GetPgClient
{
    fn delete_by_id(&self, id: i32) -> Result<bool> {
        self.get_pg_client()
            .execute(
                &*format!("delete from {} where id = $1;", self.get_table_name()),
                &[&id]
            )
            .context("delete operation failed")
            .map(|rows_affected| rows_affected > 0)
    }
}

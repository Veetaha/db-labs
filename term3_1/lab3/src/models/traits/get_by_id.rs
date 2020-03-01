use anyhow::{Result, Context};

use super::{GetTableName, GetPgClient, EntityService};

pub trait GetById: EntityService {
    fn get_by_id(&self, id: i32) -> Result<Self::Entity>;
}

impl<T: EntityService> GetById for T
where
    Self: GetTableName + GetPgClient
{

    fn get_by_id(&self, id: i32) -> Result<Self::Entity> {
        self.get_pg_client()
            .query_one(
                &*format!("select * from {} where id = $1;", self.get_table_name()),
                &[&id]
            )
            .context("failed to get entity from database by id")
            .map(Self::Entity::from)
    }
}

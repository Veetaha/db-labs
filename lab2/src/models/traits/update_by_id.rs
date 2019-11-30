use anyhow::{Result, bail, Context};
use super::{
    GetTableName,
    GetPgClient,
    EntityService,
    GetId,
    GetColUpds,
    ColUpd,
    UpdatableEntityService
};



pub trait UpdateById: EntityService + UpdatableEntityService {
    fn update_by_id(&self, entity_upd: &Self::EntityUpd) -> Result<Self::Entity>;
}


impl<T> UpdateById for T 
where
    Self: EntityService + UpdatableEntityService + GetTableName + GetPgClient,
    Self::EntityUpd: GetId + GetColUpds 
{

    fn update_by_id(&self, upd: &Self::EntityUpd) -> Result<Self::Entity> {
        let col_upds: Vec<ColUpd<'_>> = upd.get_col_upds();
        
        if col_upds.is_empty() {
            bail!("Entity update data contains no columns to update");
        }

        let query = format!(
            "update {} set {} returning *;",
            self.get_table_name(),
            col_upds
                .iter()
                .enumerate()
                .map(|(i, v)| format!("{} = ${}", v.col, i))
                .collect::<Vec<_>>()
                .join(",")
        );

        use fallible_iterator::FallibleIterator;

        // create an explicit variable to workaround temporaries drop order error
        let entity = Self::Entity::from(self
            .get_pg_client()
            .query_raw(&query[..], col_upds.iter().map(|v| v.val))
            .context("Failed to update entity by id")?
            .next()
            .context("Failed to get the record returned by update query")?
            .context("Update record iterator appeared to be empty")?
        );
        Ok(entity)
    }
}

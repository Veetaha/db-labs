use anyhow::{Result, bail, Context};
use super::{
    GetTableName,
    GetPgClient,
    EntityService,
    GetId,
    UpdatableEntityService,
    ColDataVec
};



pub trait UpdateById: EntityService + UpdatableEntityService {
    fn update_by_id(&self, entity_upd: Self::EntityUpd) -> Result<Self::Entity>;
}


impl<T> UpdateById for T 
where
    Self: EntityService + UpdatableEntityService + GetTableName + GetPgClient,
    Self::EntityUpd: GetId + Into<ColDataVec>
{

    fn update_by_id(&self, upd: Self::EntityUpd) -> Result<Self::Entity> {
        let id = upd.get_id();
        let col_datas: ColDataVec = upd.into();
        
        if col_datas.is_empty() {
            bail!("Entity update data contains no columns to update");
        }

        let query = format!(
            "update {table} set {assignments} where id = {id} returning *;",
            table = self.get_table_name(),
            id = id,
            assignments = col_datas.col.iter()
                .enumerate()
                .map(|(i, col_name)| format!("{} = ${}", col_name, i + 1))
                .collect::<Vec<_>>()
                .join(",")
        );

        use fallible_iterator::FallibleIterator;

        // create an explicit variable to workaround temporaries drop order error
        let entity = Self::Entity::from(self
            .get_pg_client()
            .query_raw(&query[..], col_datas.val.iter().map(AsRef::as_ref))
            .context("Failed to update entity by id")?
            .next()
            .context("Failed to get the record returned by update query")?
            .context("Update record iterator appeared to be empty")?
        );
        Ok(entity)
    }
}

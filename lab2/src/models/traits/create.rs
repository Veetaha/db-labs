use crate::database::PgPlaceholdersSeq;

use anyhow::{Result, Context, bail};
use super::{EntityService, CreatableEntityService, ColDataVec, GetTableName, GetPgClient};

pub trait Create: CreatableEntityService + EntityService {
    fn create(&self, new: Self::EntityNew) -> Result<Self::Entity>;
}

impl<T> Create for T
where
    Self: CreatableEntityService + EntityService + GetTableName + GetPgClient,
    Self::EntityNew: Into<ColDataVec>
{
    fn create(&self, new: Self::EntityNew) -> Result<Self::Entity> {
        let col_datas: ColDataVec = new.into();
        
        if col_datas.is_empty() {
            bail!("Can't create entity since its column data is empty.");
        }

        let query = format!(
            "insert into {table} ({cols}) values ({values}) returning *;",
            table = self.get_table_name(),
            cols = col_datas.col.join(","),
            values = PgPlaceholdersSeq::new(1..=col_datas.len())
        );

        use fallible_iterator::FallibleIterator;

        // create an explicit variable to workaround temporaries drop order error
        let entity = Self::Entity::from(self
            .get_pg_client()
            .query_raw(&query[..], col_datas.val.iter().map(AsRef::as_ref))
            .context("Failed to create entity")?
            .next()
            .context("Failed to get the record returned by insert query")?
            .context("Insert record iterator appeared to be empty")?
        );
        Ok(entity)
    }
}

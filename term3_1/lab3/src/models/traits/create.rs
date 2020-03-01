use anyhow::{Result, Context};
use diesel::associations::HasTable;
use super::{QueryableEntityService, CreatableEntityService, GetPgConn};

pub trait Create: CreatableEntityService + QueryableEntityService {
    fn create(&self, new: Self::EntityNew) -> Result<Self::Entity>;
}

impl<T> Create for T
where
    Self: CreatableEntityService + QueryableEntityService + HasTable + GetPgConn
{
    fn create(&self, new: Self::EntityNew) -> Result<Self::Entity> {
        diesel::insert_into(self.table())
            .values(new)
            .get_result(self.get_pg_conn())
            .context("Insertion failed!")
    }
}

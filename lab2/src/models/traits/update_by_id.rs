use diesel::{prelude::*, QueryResult, pg::Pg, deserialize::QueryableByName};

use super::{GetTableName, GetPgConn, EntityService};

pub trait GetById<TEntity, TEntityUpd> {
    fn update_by_id(&self, id: i32, entity_upd: TEntityUpd) -> QueryResult<TEntity>;
}

impl<T, TEntity, TEntityUpd> GetById<TEntity, TEntityUpd> for T
where
    TEntityUpd: serde::Serialize,
    TEntity: QueryableByName<Pg>,
    Self: GetTableName + GetPgConn + EntityService<Entity = TEntity>
{
    fn update_by_id(&self, id: i32) -> QueryResult<Option<TEntity>> {
        use diesel::sql_types::*;

        diesel::sql_query(format!("select * from {} where id = $1;", self.get_table_name()))
            .bind::<Integer, _>(id) 
            .load(&*self.get_pg_conn())
            .map(|mut entity_vec: Vec<_>| entity_vec.swap_remove(0))
    }
}

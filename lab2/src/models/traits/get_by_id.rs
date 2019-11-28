use diesel::{prelude::*, QueryResult, pg::Pg, deserialize::QueryableByName};


use super::{GetTableName, GetPgConn, EntityService};

pub trait GetById<TEntity>
    where TEntity: QueryableByName<Pg>
{
    fn get_by_id(&self, id: i32) -> QueryResult<Option<TEntity>>;
}

impl<T, TEntity> GetById<TEntity> for T
where
    TEntity: QueryableByName<Pg>,
    Self: GetTableName + GetPgConn + EntityService<Entity = TEntity>
{
    fn get_by_id(&self, id: i32) -> QueryResult<Option<TEntity>> {
        use diesel::sql_types::*;

        diesel::sql_query(format!("select * from {} where id = $1;", self.get_table_name()))
            .bind::<Integer, _>(id)
            .load(&*self.get_pg_conn())
            .map(|mut entity_vec: Vec<_>| entity_vec.swap_remove(0))
    }
}

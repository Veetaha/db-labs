use anyhow::{Result, Context};
use crate::{
    models::{
        entities::News,
        traits
    },
    cli::{ NewsNew, NewsUpdate, NewsSearch },
    database::PgConnPool
};

pub struct NewsService {
    pg_conn_pool: PgConnPool
}

impl NewsService {
    pub fn new(pg_conn_pool: PgConnPool) -> Self {
        Self { pg_conn_pool }
    }

    pub fn search(&self, search: &NewsSearch) -> Result<Vec<News>> {
        if let Some(body) = search.body {
            use traits::*;

            let mut client = self.get_pg_client();

            let query = format!(
                "select *{ts_headline}\
                from {table}{fulltext_query}\
                where {conditions};\
                ",
                table          = self.get_table_name(),
                ts_headline    = ", ts_headline(body, q)",
                fulltext_query = ", plainto_tsquery('english', $1) as q",
                conditions     = "to_tsvector('english', body) @@ q"
            );

            // use fallible_iterator::FallibleIterator;

            unimplemented!();
            // return client.query_raw(query, &[&body])?
            //     .map(|row| Ok(News::from(row)))
            //     .collect();
        }

        // let conditions = "";

        // let join = if search.max_likes.is_some() || search.min_likes.is_some()

        
        unimplemented!();
    }

}

impl traits::EntityService          for NewsService { type Entity = News; }
impl traits::UpdatableEntityService for NewsService { type EntityUpd = NewsUpdate; }
impl traits::CreatableEntityService for NewsService { type EntityNew = NewsNew; }
impl traits::GetTableName  for NewsService { fn get_table_name(&self) -> &str { "news" } }
impl traits::GetPgConnPool for NewsService {
    fn get_pg_conn_pool(&self) -> PgConnPool {
        // Clone operation is fast and efficient since it is just an atomic reference counter
        // under the hood, thus to make it explicit calling `clone()`
        // not as a metod but via free function call syntax to make it explicit
        PgConnPool::clone(&self.pg_conn_pool)
    }
}
impl traits::GetPgClientFromPoolInfallible for NewsService {}

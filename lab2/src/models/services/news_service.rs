use soa_derive::StructOfArray;
use anyhow::{Result, Context};
use fallible_iterator::{FromFallibleIterator, IntoFallibleIterator, FallibleIterator};
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

#[derive(Debug, StructOfArray)]
#[soa_derive = "Debug"]
pub struct NewsSearchResult {
    news: News,
    highlighted_body: Option<String>
}
impl From<pg::Row> for NewsSearchResult {
    fn from(row: pg::Row) -> Self { Self {
        highlighted_body: row.get("highlighted_body"),
        news: News::from(row)
    } }
}

impl FromFallibleIterator<pg::Row> for NewsSearchResultVec {
    
    fn from_fallible_iter<I>(rows_iterator: I) -> Result<Self, I::Error>
    where
        I: IntoFallibleIterator<Item = pg::Row> 
    {
        let mut rows_iterator = rows_iterator.into_fallible_iter();

        let mut result = NewsSearchResultVec::new();
        
        while let Some(row) = rows_iterator.next()? {
            result.push(NewsSearchResult::from(row));
        }

        Ok(result)
    }
}


impl NewsService {
    pub fn new(pg_conn_pool: PgConnPool) -> Self {
        Self { pg_conn_pool }
    }

    pub fn search(&self, search: &NewsSearch) -> Result<NewsSearchResultVec> {
        use traits::*;
        use crate::database::SqlParams;

        let mut select_clauses = vec!["*".to_owned()];
        let mut where_clauses  = Vec::new();
        let mut from_clauses   = vec![self.get_table_name().to_owned()];

        let mut params = SqlParams::new();

        let mut add_likes_bound = |operator, rhs| where_clauses.push(format!(
            "(\
                select likes from news_rating_counts_view \
                where {news_table}.id = news_rating_counts_view.news_id \
             ) {operator} ${max_likes}::int::bigint",
            news_table = self.get_table_name(),
            operator = operator,
            max_likes = params.push(rhs)
        ));

        if let Some(max_likes) = &search.max_likes {
            add_likes_bound("<=", max_likes);
        }
        if let Some(min_likes) = &search.min_likes {
            add_likes_bound(">=", min_likes);
        }
        
        if let Some(titles) = &search.title {
            use crate::database::PgPlaceholdersSeq;
            
            where_clauses.push(format!("title in ({})", PgPlaceholdersSeq::new(
                params.len()..=(params.len() + titles.len())
            )));
            // this map is necessary in because it maps regular &String
            // to fat references (polymorphic ones that contain vtable pointer for ToSql)
            // &String: (ptr) -> &dyn ToSql: (ptr, vtable)
            params.extend(titles.iter().map(|t| t as _));
        }
        let not_body_val;
        if let Some(not_body) = &search.not_body {
            not_body_val = format!("!({})", not_body
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" | ")
            );

            where_clauses.push(format!(
                "to_tsvector('english', body) @@ to_tsquery('english', ${})",
                params.push(&not_body_val)
            ));
        }

        if let Some(body) = &search.body {
            let body_param = params.push(body);
            select_clauses.push(format!(
                "ts_headline(${}, q, 'StartSel = [, \
                StopSel = ], HighlightAll = true') as highlighted_body", 
                body_param
            ));
            from_clauses.push(format!("plainto_tsquery('english', ${}) as q", body_param));
            where_clauses.push("to_tsvector('english', body) @@ q".to_owned());
        }

        let mut client = self.get_pg_client();

        let query = format!(
            "select {select_clause} from {from_clause}{where}{where_clause};",
            select_clause = select_clauses.join(","),
            from_clause   = from_clauses.join(","),
            where         = if where_clauses.is_empty() { " " } else { " where " },
            where_clause  = where_clauses.join(" and ")
        );

        let params = params.as_slice()
            .into_iter()
            .map(std::ops::Deref::deref);

        dbg!(&query);
        dbg!(&params);

        // IntoIterator
        let res = client.query_raw(&*query, params)?
            .collect()
            .context("News service failed to search news.");

        res
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

use soa_derive::StructOfArray;
use anyhow::{Result, Context};
use diesel::prelude::*;

use crate::{
    cli,
    models::{
        schema::news::dsl as DieselNewsDsl,
        entities::{self, News}
    },
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

impl NewsService {
    pub fn new(pg_conn_pool: PgConnPool) -> Self {
        Self { pg_conn_pool }
    }

    pub fn create(&self, news_new: cli::NewsNew) -> Result<News> {
        use DieselNewsDsl::*;

        diesel::insert_into(news)
            .values(&news_new)
            .get_result(&*self.pg_conn_pool.get().unwrap())
            .context("Failed to create news")
    }

    pub fn get_by_id(&self, target_id: i32) -> Result<News> {
        use DieselNewsDsl::*;

        news.filter(id.eq(target_id))
            .get_result(&*self.pg_conn_pool.get().unwrap())
            .context("Failed to get news by id")
    }

    pub fn update_by_id(&self, upd: cli::NewsUpdate) -> Result<News> {
        entities::NewsUpdate::from(upd)
            .save_changes(&*self.pg_conn_pool.get().unwrap())
            .context("Failed to update news by id")
    }

    pub fn delete_by_id(&self, target_id: i32) -> Result<bool> {
        use DieselNewsDsl::*;

        diesel::delete(news).filter(id.eq(target_id))
            .execute(&*self.pg_conn_pool.get().unwrap())
            .map(|rows_affected| rows_affected > 0)
            .context("Failed to delte news by id")
    }
/*
    pub fn search(&self, search: &NewsSearch) -> Result<NewsSearchResultVec> {
        use traits::*;

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
    }*/

}

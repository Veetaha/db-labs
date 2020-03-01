use chrono::NaiveDateTime;
use diesel::{Queryable, Identifiable};
use crate::models::schema::news;

#[derive(Debug, Queryable, Identifiable)]
#[table_name="news"]
pub struct News {
    id: i32,
    creator_id: i32,
    body: String,
    promo_img_id: Option<String>,
    last_update_date: NaiveDateTime,
    creation_date: NaiveDateTime,
    title: String
}


#[derive(Debug, Identifiable, AsChangeset)]
#[table_name="news"]
pub struct NewsUpdate {
    pub id: i32,
    pub body: Option<String>,
    pub promo_img_id: Option<Option<String>>,
    pub title: Option<String>,
}

impl From<crate::cli::NewsUpdate> for NewsUpdate {
    
    fn from(cli_upd: crate::cli::NewsUpdate) -> Self { Self {
        id:           cli_upd.id,
        body:         cli_upd.data.body,
        promo_img_id: cli_upd.data.promo_img_id,
        title:        cli_upd.data.title
    }}

}



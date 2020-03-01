use chrono::NaiveDateTime;
use pg::row::Row as PgRow;

#[derive(Debug)]
pub struct News {
    id: i32,
    creator_id: i32,
    body: String,
    promo_img_id: Option<String>,
    last_update_date: NaiveDateTime,
    creation_date: NaiveDateTime,
    title: String
}

impl From<PgRow> for News {
    fn from(row: PgRow) -> Self { Self {
        id:               row.get("id"),
        creator_id:       row.get("creator_id"),
        body:             row.get("body"),
        promo_img_id:     row.get("promo_img_id"),
        last_update_date: row.get("last_update_date"),
        creation_date:    row.get("creation_date"),
        title:            row.get("title"),
    }}
}

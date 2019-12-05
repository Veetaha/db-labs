use pg::row::Row as PgRow;
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
#[postgres(name="ratingvalue")]
pub enum RatingValue {
    Like, Dislike
}

#[derive(Debug)]
pub struct NewsRating {
    pub id: i32,
    pub rater_id: i32,
    pub news_id: i32,
    pub value: RatingValue
}


impl From<PgRow> for NewsRating {
    fn from(row: PgRow) -> Self { Self {
        id:       row.get("id"),
        rater_id: row.get("rater_id"),
        news_id:  row.get("news_id"),
        value:    row.get("value"),
    }}
}

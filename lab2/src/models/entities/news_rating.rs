use diesel::{QueryableByName};
use diesel_derive_enum::DbEnum;

use crate::schema::news_ratings;

#[derive(Debug, DbEnum)]
pub enum RatingValue {
    Like, Dislike
}

#[derive(QueryableByName, Debug)]
#[table_name = "news_ratings"]
pub struct NewsRating {
    pub id: i32,
    pub rater_id: i32,
    pub news_id: i32,
    pub value: RatingValue
}

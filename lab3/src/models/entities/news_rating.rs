use diesel_derive_enum::DbEnum;
use diesel::{Queryable, Insertable};

use crate::{cli, models::schema::news_ratings};

#[derive(Debug, DbEnum)]
#[PgType = "userrole"]
pub enum RatingValue {
    #[db_rename="like"]
    Like,
    #[db_rename="dislike"]
    Dislike
}

#[derive(Debug, Queryable)]
pub struct NewsRating {
    pub id: i32,
    pub rater_id: i32,
    pub news_id: i32,
    pub value: RatingValue
}

#[derive(Debug, Insertable)]
#[table_name="news_ratings"]
pub struct NewsRatingNew {
    pub rater_id: i32,
    pub news_id: i32,
    pub value: RatingValue
}

impl From<cli::NewsRatingNew> for NewsRatingNew {
    fn from(cli_rating: cli::NewsRatingNew) -> Self { Self {
        rater_id: cli_rating.rater_id,
        news_id:  cli_rating.news_id,
        value: if cli_rating.like { RatingValue::Like } else { RatingValue::Dislike }
    }}
}


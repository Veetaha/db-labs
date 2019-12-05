use structopt::StructOpt;

use crate::models::traits::{ColData, ColDataVec};

#[derive(StructOpt, Debug)]
pub struct NewsRatingNew {
    /// Id of user who created the rating
    #[structopt(long)]
    pub rater_id: i32,

    // Id of news to put the rating for
    #[structopt(long)]
    pub news_id: i32,

    // Like if presents, dislike otherwise
    #[structopt(long)]
    pub like: bool
}


impl From<NewsRatingNew> for ColDataVec {

    fn from(new_rating: NewsRatingNew) -> Self {
        let mut cols = Self::new();

        cols.push(ColData::with_boxed("rater_id", new_rating.rater_id));
        cols.push(ColData::with_boxed("news_id",  new_rating.news_id));
        cols.push(ColData::with_boxed("like", {
            use crate::models::entities::RatingValue;
            if new_rating.like {
                RatingValue::Like
            } else {
                RatingValue::Dislike
            }
        }));

        cols
    }

}


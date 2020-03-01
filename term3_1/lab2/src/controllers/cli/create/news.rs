use structopt::StructOpt;

use crate::models::traits::{ColData, ColDataVec};

#[derive(StructOpt, Debug)]
pub struct NewsNew {
    /// News creator id
    #[structopt(long)]
    pub creator_id: i32,

    /// News body
    #[structopt(long)]
    pub body: String,

    /// News title
    #[structopt(long)]
    pub title: String,

    /// Optional news promo image id
    #[structopt(long)]
    pub promo_img_id: Option<String>,
}


impl From<NewsNew> for ColDataVec {

    fn from(news_new: NewsNew) -> Self {
        let mut cols = Self::new();

        cols.push(ColData::with_boxed("creator_id",   news_new.creator_id));
        cols.push(ColData::with_boxed("body",         news_new.body));
        cols.push(ColData::with_boxed("title",        news_new.title));
        cols.push(ColData::with_boxed("promo_img_id", news_new.promo_img_id));

        cols
    }

}


use diesel::Insertable;
use structopt::StructOpt;

use crate::models::schema::news;

#[derive(StructOpt, Debug, Insertable)]
#[table_name="news"]
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


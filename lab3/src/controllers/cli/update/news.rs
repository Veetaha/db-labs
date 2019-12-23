use structopt::{StructOpt, clap::ArgGroup};


#[derive(StructOpt, Debug)]
pub struct NewsUpdate {
    /// Unique identifier of the entity to update.
    pub id: i32,

    #[structopt(flatten)]
    pub data: NewsUpdateData
}

#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("news_upd").required(true).multiple(true))]
pub struct NewsUpdateData {
    /// Set new news body
    #[structopt(long, group = "news_upd")]
    pub body: Option<String>,

    /// Set new promo image of news (remove promo image if value is omitted)
    #[structopt(long, group = "news_upd")]
    pub promo_img_id: Option<Option<String>>,

    /// Set new news title
    #[structopt(long, group = "news_upd")]
    pub title: Option<String>,
}


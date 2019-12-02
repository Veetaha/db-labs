use structopt::StructOpt;

use crate::models::traits::{ColData, ColDataVec, GetId};

#[derive(StructOpt, Debug)]
pub struct NewsUpdate {
    /// Unique identifier of the entity to update.
    pub id: i32,

    #[structopt(flatten)]
    pub data: NewsUpdateData
}

// TODO: add ArgGroup (see todo in update/user.rs)
#[derive(StructOpt, Debug)]
pub struct NewsUpdateData {
    /// Set new news body
    #[structopt(long)]
    pub body: Option<String>,

    /// Set new promo image of news (remove promo image if value is omitted)
    #[structopt(long)]
    pub promo_img_id: Option<Option<String>>,

    /// Set new news title
    #[structopt(long)]
    pub title: Option<String>,
}

impl GetId for NewsUpdate {
    fn get_id(&self) -> i32 { self.id }
}

impl From<NewsUpdate> for ColDataVec {

    fn from(NewsUpdate { data, .. }: NewsUpdate) -> Self {
        let mut cols = Self::new();

        ColData::try_push(&mut cols, "body",         data.body);
        ColData::try_push(&mut cols, "promo_img_id", data.promo_img_id);
        ColData::try_push(&mut cols, "title",        data.title);

        return cols;
    }
}

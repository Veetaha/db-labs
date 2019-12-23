use structopt::StructOpt;

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


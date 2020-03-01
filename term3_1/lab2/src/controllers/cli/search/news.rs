use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct NewsSearch {

    /// Set the minimum amount of likes under the news to be returned
    #[structopt(long)]
    pub min_likes: Option<i32>,

    /// Set the maximum amount of likes under the news to be returned
    #[structopt(long)]
    pub max_likes: Option<i32>,

    /// Sets the enumeration of possible titles that the returned news must have
    #[structopt(long)]
    pub title: Option<Vec<String>>,

    /// Any string to be used in fulltext search query, such that
    /// news that contain it should be excluded from result set
    #[structopt(long)]
    pub body: Option<String>,

    /// Any string to be used in fulltext search query, such that
    /// news that contain it should be excluded from result set
    #[structopt(long)]
    pub not_body: Option<String>
}

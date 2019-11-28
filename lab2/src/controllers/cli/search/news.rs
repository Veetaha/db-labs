use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct News {

    /// Set the minimum amount of likes under the news to be returned
    #[structopt(long)]
    min_likes: Option<u32>,

    /// Set the maximum amount of likes under the news to be returned
    #[structopt(long)]
    max_likes: Option<u32>,

    /// Sets the enumeration of possible titles that the returned news must have
    #[structopt(short, long)]
    title: Vec<String>,

    /// Any string be used in fulltext search query by news body
    body: String
}

mod news;

pub use news::*;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Search {
    /// Search news (with advanced filtering and fulltext search capabilities)
    News(news::NewsSearch)
}


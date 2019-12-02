mod news;

pub use news::*;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Search {
    News(news::NewsSearch)
}


mod user;
mod news;

use structopt::StructOpt;

pub use user::*;
pub use news::*;

#[derive(StructOpt, Debug)]
pub enum Update {
    User(user::UserUpdate),
    News(news::NewsUpdate)
}


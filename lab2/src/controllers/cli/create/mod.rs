mod user;
mod news;

use structopt::StructOpt;

pub use user::UserNew;
pub use news::NewsNew;

#[derive(StructOpt, Debug)]
pub enum Create {
    User(UserNew),
    News(NewsNew)
}

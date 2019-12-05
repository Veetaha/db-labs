mod user;
mod news;
mod news_rating;

use structopt::StructOpt;

pub use user::UserNew;
pub use news::NewsNew;
pub use news_rating::NewsRatingNew;

#[derive(StructOpt, Debug)]
pub enum Create {
    User(UserNew),
    News(NewsNew),
    NewsRating(NewsRatingNew),
}

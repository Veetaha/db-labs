mod user_view;
mod news_view;

pub use user_view::*;
pub use news_view::*;

pub fn display_err(err: &impl std::fmt::Debug) {
    eprintln!("Error: {:?}", err);
}

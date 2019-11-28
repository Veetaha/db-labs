mod user_view;

pub use user_view::*;

pub fn display_err(err: &impl std::fmt::Debug) {
    eprintln!("Error: {:?}", err);
}

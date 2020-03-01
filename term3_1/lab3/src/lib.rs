///! This is the documentation comment for this crate.
///! It supports **markdown**. Anything that is written here
///! will appear as the first page of the documentation to this crate.
///! lib.rs file is the entrypoint of the static library (there is no support
///! for dynamically-linked rust-native libraries untill Rust stabilizes its ABI).
///! It is totally independent from main.rs and it has no `main()` function.

#[macro_use]
extern crate diesel;

/*
 * This declares that module config is defined by "config.rs" file.
 * The name of the module **must** be the same as the name of the file.
 * 
 * You can also create nested modules in subdirectories.
 * See modules example
 */
mod config;
mod controllers;
mod views;
mod models;
mod database;


/*
 * pub(crate) correspons to `internal` C# visibility qualifier.
 * However it is more flexible, because you are free to specify actually any
 * path you want the target symbols to be accessible in.
 * Example:
 *  pub(super) - visible in the parent module
 *  pub(models::news) - visible only in models::news module
 */
pub(crate) use views::*;
pub use models::*;
pub use controllers::*;
pub use config::*;
pub use database::*;

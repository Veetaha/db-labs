mod get;
mod create;
mod update;
mod delete;
mod search;
mod push_random_entities;
mod enums;

pub use get::*;
pub use create::*;
pub use update::*;
pub use delete::*;
pub use search::*;
pub use push_random_entities::*;
pub use enums::*;

use structopt::StructOpt;

#[structopt(
    name = "Database laboratory work #2",
    about = "A simple CRUD application that interacts with a PostgreSQL database.",
    author = "Veetaha (https://github.com/Veetaha)"
)]
#[derive(StructOpt, Debug)]
pub enum Params {
    /// Retrieves entity by the given id
    ///
    /// Reaches out to PostrgreSQL database with a SELECT query
    /// passing the given id parameter to it. Then displays the specifed entity
    /// type writing it to stdout.
    Get(get::Get),

    /// Updates entity
    ///
    /// Reaches out to PostrgreSQL database with an UPDATE query
    /// passing the given id parameter and sepecified property values to it.
    /// Then displays the resulting entity after update writing it to stdout.
    Update(update::Update),

    /// Creates entity
    ///
    /// Reaches out to PostrgreSQL database with a CREATE query
    /// passing the given sepecified initial property values to it.
    /// Then displays the resulting entity after create writing it to stdout.
    Create(create::Create),

    /// Deletes entity
    ///
    /// Reaches out to PostrgreSQL database with a DELETE query
    /// passing the given id parameter.
    Delete(delete::Delete),

    /// Search entities
    Search(search::Search),

    /// Pushes random amount of random entities to all tables in the database
    PushRandomEntities(push_random_entities::PushRandomEntities)
}

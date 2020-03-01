use structopt::StructOpt;

use super::enums::EntityType;


#[derive(StructOpt, Debug)]
pub struct Get {
    #[structopt(
        possible_values = &EntityType::variants(),
        case_insensitive = true
    )]
    pub entity: EntityType,

    /// Unique identifier of the entity to search for
    pub id: i32
}

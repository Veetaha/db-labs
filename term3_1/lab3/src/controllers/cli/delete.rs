use structopt::StructOpt;

use super::enums::EntityType;

#[derive(StructOpt, Debug)]
pub struct Delete {
    #[structopt(
        case_insensitive = true,
        possible_values = &EntityType::variants()
    )]
    pub entity: EntityType,

    /// Unique identifier of the entity to delete
    pub id: i32
}

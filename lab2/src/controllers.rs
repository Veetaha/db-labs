use structopt::{StructOpt, clap::arg_enum};

use diesel::prelude::*;

arg_enum! {
    #[derive(
        StructOpt,
        Debug
    )]
    pub enum EntityType { User, News, NewsComment, NewsRating }
}


#[structopt(
    name = "Database laboratory work #2",
    about = "A simple CRUD application that interacts with a PostgreSQL database.",
    author = "Veetaha (https://github.com/Veetaha)"
)]
#[derive(StructOpt, Debug)]
pub enum CliParams {
    /// Retrieves news by the given id
    ///
    /// Reaches out to PostrgreSQL database with a SELECT query
    /// passing the given id parameter to it. Then displays the specifed entity
    /// type writing it to stdout.
    #[structopt()]
    Get {
        
        #[structopt(
            possible_values = &EntityType::variants(),
            case_insensitive = true
        )]
        entity: EntityType,

        /// Unique identifier of the entity to search for
        id: u64
    }
}

pub fn run(db_conn: diesel::PgConnection, params: CliParams) {

    let ret = db_conn.execute(
        "insert into users (password_hash, login, name, role)\
         values('aaaa', 'veetaha_first', 'veetaha', 'Admin')
        "
    ).expect("Failed =(");

    dbg!(ret);

}

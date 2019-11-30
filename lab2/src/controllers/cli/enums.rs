use postgres_types::ToSql;
use structopt::{StructOpt, clap::arg_enum};

arg_enum! {
    /// User role defines it's access rights
    #[derive(StructOpt, Debug, ToSql)]
    pub enum UserRole { Admin, Regular, Guest }
}

arg_enum! {
    /// Type of entity the operation is performed on.
    #[derive(StructOpt,Debug)]
    pub enum EntityType { User, News, NewsComment, NewsRating }
}

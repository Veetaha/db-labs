mod users_controller;
pub mod cli;

use anyhow::Result;
use users_controller::{UserController};
use crate::{
    services::UserService,
    database::PgConnPool
};

/*
 * Composition and execution root.
 */
pub fn run(pg_conn_pool: PgConnPool, params: cli::Params) -> Result<()> {

    let user_controller = || UserController::new(UserService::new(pg_conn_pool));

    use cli::{Params as P, EntityType as ET, Get, Create, Update, Delete};

    match params {
        P::Get   (Get    {entity: ET::User, id}) => user_controller().get_by_id(id),
        P::Delete(Delete {entity: ET::User, id}) => user_controller().delete(id),
        P::Create(Create::User(new_user))        => user_controller().create(&new_user),
        P::Update(Update::User(user_update))     => user_controller().update(&user_update),
        
        
        _ => unimplemented!()
    }

    Ok(())
}

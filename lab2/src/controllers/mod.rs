mod users_controller;
mod news_controller;
pub mod cli;

use anyhow::Result;

use users_controller::UserController;
use news_controller::NewsController;
use crate::{
    services::{UserService, NewsService},
    database::PgConnPool
};

/*
 * Composition and execution root.
 */
pub fn run(pg_conn_pool: PgConnPool, params: cli::Params) -> Result<()> {

    let user_controller = || UserController::new(UserService::new(pg_conn_pool.clone()));
    let news_controller = || NewsController::new(NewsService::new(pg_conn_pool.clone()));

    use cli::{
        Params as P,
        EntityType as ET,
        Get, Create, Update, Delete, Search
    };

    match params {
        P::Get(Get {entity: ET::User, id}) => user_controller().get_by_id(id),
        P::Get(Get {entity: ET::News, id}) => news_controller().get_by_id(id),

        P::Delete(Delete {entity: ET::User, id}) => user_controller().delete(id),
        P::Delete(Delete {entity: ET::News, id}) => news_controller().delete(id),

        P::Create(Create::User(new)) => user_controller().create(new),
        P::Create(Create::News(new)) => news_controller().create(new),

        P::Update(Update::User(upd)) => user_controller().update(upd),
        P::Update(Update::News(upd)) => news_controller().update(upd),
        
        P::Search(Search::News(news_search)) => news_controller().search(&news_search),

        _ => unimplemented!()
    }

    Ok(())
}

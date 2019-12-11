mod users_controller;
mod news_controller;
mod global_controller;
mod news_ratings_controller;
pub mod cli;

use anyhow::Result;

use users_controller::UserController;
use news_controller::NewsController;
use global_controller::GlobalController;
use news_ratings_controller::NewsRatingController;
use crate::{
    services::{UserService, NewsService, GlobalDbService, NewsRatingService},
    database::PgConnPool
};

/*
 * Composition and execution root.
 */
pub fn run(pg_conn_pool: PgConnPool, params: cli::Params) -> Result<()> {
    let global_controller = || GlobalController::new(GlobalDbService::new(PgConnPool::clone(&pg_conn_pool)));
    let user_controller = || UserController::new(UserService::new(PgConnPool::clone(&pg_conn_pool)));
    let news_controller = || NewsController::new(NewsService::new(PgConnPool::clone(&pg_conn_pool)));
    let news_rating_controller = || NewsRatingController::new(NewsRatingService::new(PgConnPool::clone(&pg_conn_pool)));

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
        P::Create(Create::NewsRating(new)) => news_rating_controller().create(new),

        P::Update(Update::User(upd)) => user_controller().update(upd),
        P::Update(Update::News(upd)) => news_controller().update(upd),
        
        P::Search(Search::News(news_search)) => news_controller().search(&news_search),
    
        P::PushRandomEntities(opts) => global_controller().push_random_entities(opts),

        _ => unimplemented!()
    }

    Ok(())
}

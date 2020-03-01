use anyhow::Context;

use crate::{
    models::{
        services::NewsService,
        traits::*
    },
    views,
    cli
};

pub struct NewsController {
    news: NewsService
}

impl NewsController {
    pub fn new(news: NewsService) -> Self {
        Self { news }
    }

    pub fn search(&self, search: &cli::NewsSearch) {
        let news = self.news.search(search)
            .context("News service failed to find news");
        
        match &news {
            Ok(news) => views::display_found_news(news.iter()),
            Err(err) => views::display_err(err)
        }
    }


    pub fn get_by_id(&self, id: i32) {
        let news = self.news.get_by_id(id)
            .context("News service failed to fetch news");
        
        match &news {
            Ok(news) => views::display_news_by_id(news),
            Err(err) => views::display_err(err)
        }
    }

    pub fn create(&self, input_news: cli::NewsNew) {
        let news = self.news.create(input_news)
            .context("News service failed to create news");

        match &news {
            Ok(news) => views::display_new_news(news),
            Err(err) => views::display_err(err)
        }
    }

    pub fn update(&self, news_update: cli::NewsUpdate) {
        let news = self.news.update_by_id(news_update)
            .context("news service failed to update news");

        match &news {
            Ok(news) => views::display_updated_news(news),
            Err(err) => views::display_err(err)
        }
    }

    pub fn delete(&self, id: i32) {
        let were_deleted = self.news.delete_by_id(id);

        match were_deleted {
            Ok(were_deleted) => views::display_news_were_deleted(id, were_deleted),
            Err(err) => views::display_err(&err)
        }
    }
}

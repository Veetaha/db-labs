use anyhow::Context;

use crate::{
    models::{
        services::NewsRatingService,
        traits::*
    },
    views,
    cli
};

pub struct NewsRatingController {
    ratings: NewsRatingService
}

impl NewsRatingController {
    pub fn new(ratings: NewsRatingService) -> Self {
        Self { ratings }
    }

    pub fn create(&self, input_rating: cli::NewsRatingNew) {
        let rating = self.ratings.create(input_rating)
            .context("News rating service failed to create news");

        match &rating {
            Ok(rating) => views::display_new_news_rating(rating),
            Err(err) => views::display_err(err)
        }
    }

}

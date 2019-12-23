use crate::models::{
    services::NewsSearchResultRef,
    entities::News
};

pub fn display_found_news<'a>(news: impl Iterator<Item = NewsSearchResultRef<'a>>) {
    println!("Successfully found news:");
    // TODO: Add fulltext query fragment emphasis
    for item in news {
        println!("{:#?}", item);
    }
}

pub fn display_news_by_id(news: &News) {
    println!("Retrieved news by id: {:#?}", news);
}

pub fn display_new_news(news: &News) {
    println!("News were created successfully: {:#?}", news);
}

pub fn display_updated_news(news: &News) {
    println!("News were updated successfully: {:#?}", news);
}

pub fn display_news_were_deleted(id: i32, was_deleted: bool) {
    println!(
        "News under id {id} were {indeed} deleted", 
        id = id, 
        indeed = if was_deleted { "successfully" } else { "not" }
    );
}

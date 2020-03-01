alter table news_ratings add constraint unique_news_and_rater_id unique (news_id, rater_id);

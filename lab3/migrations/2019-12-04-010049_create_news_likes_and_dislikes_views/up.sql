create view news_rating_counts_view as
select
    news_id,
    count(case when value = 'like'    then 1 end) as likes,
    count(case when value = 'dislike' then 1 end) as dislikes
from news_ratings
group by news_id;

-- create function get_news_rating_count(
--     target_news_id int,
--     target_rating RatingValue
-- ) returns bigint as $$
--     select count(*)
--     from news_ratings
--     where news_ratings.news_id = target_news_id and value = target_value;
-- $$ language sql stable;

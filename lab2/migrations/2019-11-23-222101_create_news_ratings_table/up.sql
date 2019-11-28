create type RatingValue as enum (
    'like',
    'dislike'
);

create table news_ratings (
    id serial primary key,
    rater_id integer not null references users(id),
    news_id integer not null references news(id),

    value RatingValue not null
);

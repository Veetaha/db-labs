create table if not exists news_comments (
    id serial primary key,
    commentator_id integer not null references users(id),
    news_id integer not null references news(id),

    body text not null,
    creation_date timestamp not null default now(),
    last_update_date timestamp not null default now()
);

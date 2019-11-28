create table if not exists news (
    id serial primary key,
    creator_id integer not null references users(id) on delete cascade,

    body text not null,
    promo_img_id text,
    last_update_date timestamp not null default now(),
    creation_date    timestamp not null default now()
);

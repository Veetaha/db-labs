create type role as enum (
    'Regular',
    'Admin',
    'Guest'
);

create table if not exists users (
    id serial primary key,

    password_hash text not null,
    avatar_img_id text,
    login text not null unique,
    name  text not null,
    role  role not null,
    last_update_date timestamp not null default now(),
    creation_date    timestamp not null default now()
);

create index user_creation_date_index on users using brin (creation_date);
create index user_login_index on users using btree (login);
create index user_name_index on users using btree (name);

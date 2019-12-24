create function my_users_trigger() returns trigger as $$
    declare
        news_name text;
        cur_name news_ratings.news_id%TYPE;
    begin
        raise notice 'New role %', new.role;
        if new.role = 'guest'::userrole then
            raise 'Failed to set "guest" role for %. This role is prohibited for real users.', new.name;
        else
            raise notice 'User % now likes everything!', new.name;
            update news_ratings set value = 'like' where rater_id = new.id;
        end if;
        for news_name in
            select title from news
            join news_ratings on new.id = rater_id and news_id = news.id
        loop

            raise notice 'User % now likes %', new.name, news_name;
        end loop;
        return new;
    exception
            when no_data_found then
            when too_many_rows then
                raise 'Omg, Danila ty sho!';
    end;
$$ language plpgsql;

create trigger my_trigger_on_users after update or insert on users for each row execute procedure my_users_trigger();


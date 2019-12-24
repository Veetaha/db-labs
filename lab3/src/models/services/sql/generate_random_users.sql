with last_id as (select max(id) from users)

insert into users(password_hash, avatar_img_id, login, name, role)
select
    random_text(15) as password_hash,
    random_text(15) as avatar_img_id,
    'user_' || seq || '@' || (
        case (random() * 3)::int
            when 0 then 'gmail.com'
            when 1 then 'hotmail.com'
            when 2 then 'yahoo.com'
            when 3 then 'ex.ua'
        end
    ) as login,
    (case (random() * 5)::int
        when 0 then 'Viktor Petrovich'
        when 1 then 'Elena Anatilievna'
        when 2 then 'Ruslan Anatolich'
        when 3 then 'Vitoria Batkovna'
        when 4 then 'Nataliya Volodymyrivna'
        when 5 then 'Veetaha'
    end) as name,
    (case (random())::int
        when 0 then 'regular'::UserRole
        when 1 then 'admin'::UserRole
    end) as role
from GENERATE_SERIES(
    (select max(id) from users) + 1,
    (select max(id) from users) + $1
) as seq;

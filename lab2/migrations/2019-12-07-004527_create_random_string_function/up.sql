create or replace function create_random_string(int) returns text as $$
    select array_to_string(
        array(
            select substring(
                '23456789abcdefghjkmnpqrstuvwxyz'
                from floor(random() * 31)::int + 1 for 1
            )
            from generate_series(1, $1)
        ),
        ''
    );
$$
language sql;

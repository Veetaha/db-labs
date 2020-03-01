alter table news add column title text not null default '';
alter table news alter column title drop default;

-- Your SQL goes here

create table posts (
    id serial primary key,
    user_id int not null default 0,
    title varchar(255) not null default '',
    content text not null default ''
);
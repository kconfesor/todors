-- Your SQL goes here
create table todo
(
    id          serial                not null
        constraint todo_pk
            primary key,
    completed   boolean default false not null,
    created_at  timestamptz           not null,
    updated_at  timestamptz,
    title       varchar,
    description text,
    due_at      timestamptz,
    enabled     boolean default true  not null,
    user_email  varchar(100)
);
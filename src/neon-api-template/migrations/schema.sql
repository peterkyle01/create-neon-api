create extension if not exists "uuid-ossp";

create table if not exists users (
    id            uuid primary key default uuid_generate_v4(),
    email         varchar(255) unique not null,
    password_hash varchar(255) not null,
    first_name    varchar(100) not null,
    last_name     varchar(100) not null,
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now()
);

create index if not exists idx_users_email on users (email);
create index if not exists idx_users_created_at on users (created_at);

alter table users enable row level security;

create policy "allow_insert_for_all" on users
    for insert with check (true);

create policy "allow_select_own_row" on users
    for select using (auth.user_id() = id::text);

create policy "allow_update_own_row" on users
    for update using (auth.user_id() = id::text);

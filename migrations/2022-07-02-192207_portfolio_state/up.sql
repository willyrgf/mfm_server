-- Your SQL goes here
create extension if not exists "uuid-ossp";
create extension if not exists "citext";

create table portfolio_states (
	id uuid primary key default uuid_generate_v4(),
	token_id uuid not null,
	rebalancer_label citext not null,
	data jsonb not null,
	created_at timestamp default now(),
	updated_at timestamp default now()
);

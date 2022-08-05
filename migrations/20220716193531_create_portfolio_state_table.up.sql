-- Add up migration script here
create extension if not exists "uuid-ossp";

create table portfolio_states (
	id uuid primary key default uuid_generate_v4(),
	auth_token_id uuid not null,
	rebalancer_label text not null,
	data jsonb not null,
	created_at timestamp default now(),
	updated_at timestamp default now()
);

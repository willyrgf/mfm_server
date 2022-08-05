-- Add up migration script here
create table auth_tokens (
	id uuid primary key default uuid_generate_v4(),
	token uuid default uuid_generate_v4(),
	token_label text not null,
	created_at timestamp default now(),
	updated_at timestamp default now()
);

-- insert into auth_tokens (token_label) values ("my_token");

alter table portfolio_states 
	add constraint fk_auth_token 
	foreign key (auth_token_id) 
	references auth_tokens(id);

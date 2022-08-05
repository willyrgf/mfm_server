-- Add down migration script here
alter table portfolio_states 
	drop constraint fk_auth_token;

drop table auth_tokens;
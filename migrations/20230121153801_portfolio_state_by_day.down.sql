-- Add down migration script here
drop index public.idx_portfolio_states_created_at;

drop table portfolio_states_expanded_by_days;

drop function refresh_portfolio_states_expanded_by_days(interval);
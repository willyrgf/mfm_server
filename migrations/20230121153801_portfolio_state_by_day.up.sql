-- Add up migration script here
create index idx_portfolio_states_created_at on portfolio_states(created_at desc);

create table portfolio_states_expanded_by_days (
	auth_token_id uuid not null,
	token_label text not null,
	portfolio_state_id uuid not null,
	rebalancer_label text not null,
	created_at date not null,
	last_created_at timestamp not null,
	coin_balance numeric not null,
	asset_name text not null,
	asset_address text not null,
	asset_kind text not null,
	asset_network jsonb not null,
	asset_network_id text not null,
	asset_balance numeric not null,
	asset_price numeric not null,
	quoted_asset_name text not null,
	asset_quoted_balance numeric not null,
	asset_amount_to_trade numeric not null,
	quoted_amount_to_trade numeric not null
);

create unique index uidx_portfolio_states_ex_by_days on portfolio_states_expanded_by_days(auth_token_id, created_at, asset_name);

create
or replace function refresh_portfolio_states_expanded_by_days(arg_interval interval default '2 days' :: interval) 
returns void language plpgsql as
$$
begin
insert into
	portfolio_states_expanded_by_days (
		with source as (
			select
				ps.auth_token_id,
				ats.token_label,
				ps.id as portfolio_state_id,
				ps.rebalancer_label,
				ps.created_at,
				ps.created_at as last_created_at,
				(ps.data ->> 'coin_balance') :: numeric as coin_balance,
				v.value -> 'asset' ->> 'name' as asset_name,
				v.value -> 'asset' ->> 'address' as asset_address,
				v.value -> 'asset' ->> 'kind' as asset_kind,
				v.value -> 'asset' -> 'network' as asset_network,
				v.value -> 'asset' ->> 'network_id' as asset_network_id,
				(v.value ->> 'balance') :: numeric as asset_balance,
				(v.value ->> 'price') :: numeric as asset_price,
				v.value -> 'quoted_asset' ->> 'name' as quoted_asset_name,
				(v.value ->> 'quoted_balance') :: numeric as asset_quoted_balance,
				(v.value ->> 'amount_to_trade') :: numeric as asset_amount_to_trade,
				(v.value ->> 'quoted_amount_to_trade') :: numeric as quoted_amount_to_trade
			from
				portfolio_states ps
				join auth_tokens ats on ats.id = ps.auth_token_id
				left join lateral (
					select
						*
					from
						jsonb_array_elements(ps.data -> 'track_assets')
				) as v on true
			where
				now() > (ps.created_at - arg_interval)
		),
		max_min as (
			select
				max(created_at),
				min(created_at)
			from
				source
		)
		select
			s.*
		from
			source s,
			max_min mm
			left join lateral (
				select
					generate_series(mm.min :: date, mm.max :: date, '1 day' :: interval) as serie
			) as gs on true
			left join lateral (
				select
					max(s.created_at) as created_at,
					s.token_label,
					s.rebalancer_label
				from
					source s
				where
					s.created_at :: date = gs.serie
				group by
					2,
					3
			) as source_day_date_to_use on true
		where
			s.created_at = source_day_date_to_use.created_at
		order by
			s.created_at desc
	) on conflict(auth_token_id, created_at, asset_name) do
update
set
	coin_balance = excluded.coin_balance,
	asset_balance = excluded.asset_balance,
	asset_price = excluded.asset_price,
	asset_quoted_balance = excluded.asset_quoted_balance,
	asset_amount_to_trade = excluded.asset_amount_to_trade,
	quoted_amount_to_trade = excluded.quoted_amount_to_trade;

end;

$$;
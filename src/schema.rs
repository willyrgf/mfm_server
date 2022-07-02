table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;

    portfolio_states (id) {
        id -> Uuid,
        token_id -> Uuid,
        rebalancer_label -> Citext,
        data -> Jsonb,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    repetitions (id) {
        id -> Int4,
        device_id -> Nullable<Uuid>,
        set_id -> Uuid,
        session_id -> Uuid,
        rom -> Float8,
        velocity -> Float8,
        duration -> Float8,
        rep_time -> Timestamptz,
        level -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        device_id -> Uuid,
        rtfp -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    repetitions,
    users,
);

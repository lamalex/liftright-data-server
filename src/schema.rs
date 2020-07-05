table! {
    repetitions (id) {
        id -> Int4,
        device_id -> Uuid,
        session_id -> Uuid,
        set_id -> Uuid,
        exercise -> Varchar,
        rom -> Float4,
        velocity -> Float4,
        duration -> Float4,
        rep_time -> Timestamptz,
        level -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        device_id -> Uuid,
        rtfb -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    repetitions,
    users,
);

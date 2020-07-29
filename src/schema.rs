table! {
    repetitions (id) {
        id -> Int4,
        device_id -> Uuid,
        session_id -> Uuid,
        set_id -> Uuid,
        exercise -> Varchar,
        number -> Int2,
        rom -> Float4,
        velocity -> Float4,
        duration -> Float4,
        rep_time -> Timestamptz,
        level -> Varchar,
    }
}

table! {
    survey_results (id) {
        id -> Int4,
        device_id -> Uuid,
        submitted -> Timestamptz,
        survey_data -> Varchar,
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
    survey_results,
    users,
);

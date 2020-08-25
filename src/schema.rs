table! {
    imu_record_pairs (id) {
        id -> Int4,
        session_id -> Uuid,
        acc -> Int4,
        gyro -> Int4,
    }
}

table! {
    imu_records (id) {
        id -> Int4,
        x -> Float4,
        y -> Float4,
        z -> Float4,
        time -> Timestamptz,
    }
}

table! {
    repetitions (id) {
        id -> Int4,
        device_id -> Uuid,
        set_id -> Uuid,
        exercise -> Varchar,
        number -> Int2,
        rom -> Float4,
        velocity -> Float4,
        duration -> Float4,
        rep_time -> Timestamptz,
        level -> Varchar,
        session_id -> Uuid,
    }
}

table! {
    sessions (id) {
        id -> Uuid,
        device_id -> Uuid,
    }
}

table! {
    survey_results (id) {
        id -> Int4,
        device_id -> Uuid,
        submitted -> Timestamptz,
        survey_data -> Json,
    }
}

table! {
    users (id) {
        id -> Int4,
        device_id -> Uuid,
        rtfb -> Bool,
    }
}

joinable!(imu_record_pairs -> sessions (session_id));
joinable!(repetitions -> sessions (session_id));

allow_tables_to_appear_in_same_query!(
    imu_record_pairs,
    imu_records,
    repetitions,
    sessions,
    survey_results,
    users,
);

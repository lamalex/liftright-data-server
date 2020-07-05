CREATE TABLE repetitions (
    id SERIAL PRIMARY KEY,
    device_id UUID NOT NULL REFERENCES users(device_id),
    session_id UUID NOT NULL,
    set_id UUID NOT NULL,
    rom REAL NOT NULL,
    velocity REAL NOT NULL, 
    duration REAL NOT NULL,
    rep_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    level VARCHAR NOT NULL
)
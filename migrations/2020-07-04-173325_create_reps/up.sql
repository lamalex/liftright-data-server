CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    device_id UUID NOT NULL UNIQUE,
    rtfp BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE repetitions (
    id SERIAL PRIMARY KEY,
    device_id UUID REFERENCES users(device_id),
    set_id UUID NOT NULL,
    session_id UUID NOT NULL,
    rom DOUBLE PRECISION NOT NULL,
    velocity DOUBLE PRECISION NOT NULL,
    duration DOUBLE PRECISION NOT NULL,
    rep_time TIMESTAMP WITH TIME ZONE NOT NULL,
    level VARCHAR NOT NULL
);
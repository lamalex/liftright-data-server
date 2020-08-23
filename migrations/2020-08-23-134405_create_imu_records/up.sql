CREATE TABLE imu_records (
    id SERIAL PRIMARY KEY,
    x real NOT NULL,
    y real NOT NULL,
    z real NOT NULL,
    time TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE imu_pairs (
    id SERIAL PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES sessions(id),
    gyro SERIAL REFERENCES imu_records(id),
    accelerometer SERIAL REFERENCES imu_records(id)
);
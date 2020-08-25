CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    device_id UUID NOT NULL REFERENCES users(device_id)
);

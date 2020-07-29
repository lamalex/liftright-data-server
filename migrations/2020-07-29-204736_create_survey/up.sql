CREATE TABLE survey_results (
    id SERIAL PRIMARY KEY,
    device_id UUID NOT NULL REFERENCES users(device_id),
    submitted TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    survey_data JSON NOT NULL
)
ALTER TABLE repetitions DROP COLUMN session_id;
ALTER TABLE repetitions ADD COLUMN session_id UUID NOT NULL REFERENCES sessions(id);
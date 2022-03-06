-- Your SQL goes here
CREATE TABLE sensor_data(
	id SERIAL PRIMARY KEY,
    writeKey TEXT NOT NULL,
    create_at timestamptz NOT NULL DEFAULT current_timestamp,
    d1 REAL NOT NULL,
    d2 REAL NOT NULL,
    d3 REAL NOT NULL
);

-- Your SQL goes here
CREATE TABLE sensor_data(
	id SERIAL PRIMARY KEY,
    create_at timestamptz NOT NULL DEFAULT current_timestamp,
    data0 real,
    data1 real,
    data2 real
);

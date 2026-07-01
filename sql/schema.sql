DROP TABLE IF EXISTS vehicle_telemetry CASCADE;
DROP TABLE IF EXISTS vehicles CASCADE;

CREATE TABLE vehicles (
	id INT PRIMARY KEY,
	name VARCHAR(100) NOT NULL
);

CREATE TABLE vehicle_telemetry (
	id BIGSERIAL PRIMARY KEY,
	vehicle_id INT NOT NULL REFERENCES vehicles(id) ON DELETE CASCADE,

	latitude FLOAT8 NOT NULL,
	longitude FLOAT8 NOT NULL,
	altitude FLOAT8 NOT NULL,
	speed REAL NOT NULL,

	timestamp INT8 NOT NULL
);

INSERT INTO vehicles (id, name) VALUES
(1, 'truck_1'),
(2, 'truck_2');

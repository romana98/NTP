-- Your SQL goes here
CREATE TABLE IF NOT EXISTS lectures (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  number_of_times INTEGER NOT NULL
)
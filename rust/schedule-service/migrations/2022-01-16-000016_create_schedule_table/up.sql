-- Your SQL goes here
CREATE TABLE IF NOT EXISTS schedules (
  id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS assigned_shifts (
  id SERIAL PRIMARY KEY,
  schedule_id INTEGER NOT NULL,
  staff_id INTEGER NOT NULL,
  staff VARCHAR NOT NULL,
  CONSTRAINT fk_schedule  FOREIGN KEY(schedule_id) REFERENCES schedules(id)
);

CREATE TABLE IF NOT EXISTS shifts_lectures (
  id SERIAL PRIMARY KEY,
  assigned_shifts_id INTEGER NOT NULL,
  day VARCHAR NOT NULL,
  shift VARCHAR NOT NULL,
  lecture VARCHAR NOT NULL,
  CONSTRAINT fk_assigned_shifts  FOREIGN KEY(assigned_shifts_id) REFERENCES assigned_shifts(id)
);
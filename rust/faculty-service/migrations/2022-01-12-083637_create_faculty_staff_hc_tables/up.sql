-- Your SQL goes here-- Your SQL goes here-- Your SQL goes here
CREATE TABLE IF NOT EXISTS shifts (
  id SERIAL PRIMARY KEY,
  start_shift VARCHAR NOT NULL,
  end_shift VARCHAR NOT NULL,
  day VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS hard_constraints (
  id SERIAL PRIMARY KEY,
  daily_max INTEGER NOT NULL,
  weekly_max INTEGER NOT NULL,
  weekly_min INTEGER NOT NULL,
  max_per_shift INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS faculties (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  hard_constraint_id INTEGER NOT NULL,
  schedule_id INTEGER,
  CONSTRAINT fk_hard_constraint FOREIGN KEY(hard_constraint_id) REFERENCES hard_constraints(id)
);

CREATE TABLE IF NOT EXISTS shifts_faculties (
  id SERIAL PRIMARY KEY,
  faculty_id INTEGER NOT NULL,
  shift_id INTEGER NOT NULL,
  CONSTRAINT fk_faculty FOREIGN KEY(faculty_id) REFERENCES faculties(id),
  CONSTRAINT fk_shift FOREIGN KEY(shift_id) REFERENCES shifts(id)
);

CREATE TABLE IF NOT EXISTS staffs_faculties (
  id SERIAL PRIMARY KEY,
  faculty_id INTEGER NOT NULL,
  staff_id INTEGER NOT NULL,
  CONSTRAINT fk_faculty FOREIGN KEY(faculty_id) REFERENCES faculties(id)
);

CREATE TABLE IF NOT EXISTS lectures_faculties (
  id SERIAL PRIMARY KEY,
  faculty_id INTEGER NOT NULL,
  lecture_id INTEGER NOT NULL,
  CONSTRAINT fk_faculty FOREIGN KEY(faculty_id) REFERENCES faculties(id)
);
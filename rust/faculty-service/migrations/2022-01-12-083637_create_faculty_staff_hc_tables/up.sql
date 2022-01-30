-- Your SQL goes here-- Your SQL goes here-- Your SQL goes here
CREATE TABLE IF NOT EXISTS shifts (
  id SERIAL PRIMARY KEY,
  start_shift VARCHAR NOT NULL,
  end_shift VARCHAR NOT NULL,
  day VARCHAR NOT NULL
);

BEGIN;
    with data_shifts(start_shift, end_shift, day) as (
      VALUES
      ('08:00', '10:30', 'monday'),
      ('08:00', '10:30', 'tuesday'),
      ('08:00', '10:30', 'wednesday'),
      ('08:00', '10:30', 'thursday'),
      ('08:00', '10:30', 'friday'),
      ('10:45', '12:15', 'monday'),
      ('10:45', '12:15', 'tuesday'),
      ('10:45', '12:15', 'wednesday'),
      ('10:45', '12:15', 'thursday'),
      ('10:45', '12:15', 'friday'),
      ('12:30', '14:00', 'monday'),
      ('12:30', '14:00', 'tuesday'),
      ('12:30', '14:00', 'wednesday'),
      ('12:30', '14:00', 'thursday'),
      ('12:30', '14:00', 'friday'),
      ('14:15', '16:45', 'monday'),
      ('14:15', '16:45', 'tuesday'),
      ('14:15', '16:45', 'wednesday'),
      ('14:15', '16:45', 'thursday'),
      ('14:15', '16:45', 'friday')
    )
    INSERT INTO shifts (start_shift, end_shift, day) SELECT  d.start_shift, d.end_shift, d.day from data_shifts d
    WHERE NOT EXISTS (SELECT * FROM shifts);
COMMIT;

CREATE TABLE IF NOT EXISTS hard_constraints (
  id SERIAL PRIMARY KEY,
  daily_max INTEGER NOT NULL,
  weekly_max INTEGER NOT NULL,
  weekly_min INTEGER NOT NULL,
  max_per_shift INTEGER NOT NULL
);

BEGIN;
    with data_hard_constraints(daily_max, weekly_max, weekly_min, max_per_shift) as (
      VALUES
      (4, 15, 5 , 3)
    )
    INSERT INTO hard_constraints (daily_max, weekly_max, weekly_min, max_per_shift) 
    SELECT  d.daily_max, d.weekly_max, d.weekly_min, d.max_per_shift from data_hard_constraints d
    WHERE NOT EXISTS (SELECT * FROM hard_constraints);
COMMIT;

CREATE TABLE IF NOT EXISTS faculties (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  hard_constraint_id INTEGER NOT NULL,
  schedule_id INTEGER,
  CONSTRAINT fk_hard_constraint FOREIGN KEY(hard_constraint_id) REFERENCES hard_constraints(id)
);

BEGIN;
    with data_faculties(name, hard_constraint_id) as (
      VALUES
      ('ftn', 1)
    )
    INSERT INTO faculties (name, hard_constraint_id) 
    SELECT  d.name, d.hard_constraint_id from data_faculties d
    WHERE NOT EXISTS (SELECT * FROM faculties);
COMMIT;

CREATE TABLE IF NOT EXISTS shifts_faculties (
  id SERIAL PRIMARY KEY,
  faculty_id INTEGER NOT NULL,
  shift_id INTEGER NOT NULL,
  CONSTRAINT fk_faculty FOREIGN KEY(faculty_id) REFERENCES faculties(id),
  CONSTRAINT fk_shift FOREIGN KEY(shift_id) REFERENCES shifts(id)
);

BEGIN;
    with data_shifts_faculties(faculty_id, shift_id) as (
      VALUES
      (1, 1),
      (1, 2),
      (1, 3),
      (1, 4),
      (1, 5),
      (1, 6),
      (1, 7),
      (1, 8),
      (1, 9),
      (1, 10),
      (1, 11),
      (1, 12),
      (1, 13),
      (1, 14),
      (1, 15),
      (1, 16),
      (1, 17),
      (1, 18),
      (1, 19),
      (1, 20)
    )
    INSERT INTO shifts_faculties (faculty_id, shift_id) 
    SELECT  d.faculty_id, d.shift_id from data_shifts_faculties d
    WHERE NOT EXISTS (SELECT * FROM shifts_faculties);
COMMIT;

CREATE TABLE IF NOT EXISTS staffs_faculties (
  id SERIAL PRIMARY KEY,
  faculty_id INTEGER NOT NULL,
  staff_id INTEGER NOT NULL,
  CONSTRAINT fk_faculty FOREIGN KEY(faculty_id) REFERENCES faculties(id)
);

BEGIN;
    with data_staffs_faculties(faculty_id, staff_id) as (
      VALUES
      (1, 1),
      (1, 2),
      (1, 3),
      (1, 4)
    )
    INSERT INTO staffs_faculties (faculty_id, staff_id) 
    SELECT  d.faculty_id, d.staff_id from staffs_faculties d
    WHERE NOT EXISTS (SELECT * FROM staffs_faculties);
COMMIT;


CREATE TABLE IF NOT EXISTS lectures_faculties (
  id SERIAL PRIMARY KEY,
  faculty_id INTEGER NOT NULL,
  lecture_id INTEGER NOT NULL,
  CONSTRAINT fk_faculty FOREIGN KEY(faculty_id) REFERENCES faculties(id)
);

BEGIN;
    with data_lectures_faculties(faculty_id, lecture_id) as (
      VALUES
      (1, 1),
      (1, 2),
      (1, 3),
      (1, 4),
      (1, 5),
      (1, 6),
      (1, 7),
      (1, 8),
      (1, 9),
      (1, 10),
      (1, 11),
      (1, 12),
      (1, 13),
      (1, 14),
      (1, 15),
      (1, 16)
    )
    INSERT INTO lectures_faculties (faculty_id, lecture_id) 
    SELECT  d.faculty_id, d.lecture_id from data_lectures_faculties d
    WHERE NOT EXISTS (SELECT * FROM lectures_faculties);
COMMIT;

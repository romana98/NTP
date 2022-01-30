-- Your SQL goes here-- Your SQL goes here
CREATE TABLE IF NOT EXISTS admins (
  id SERIAL PRIMARY KEY,
  admin_email VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  role VARCHAR NOT NULL
);

BEGIN;
INSERT INTO admins (admin_email, password, role) VALUES ('a.admin@admin.com', '$2b$12$0KOyov06TutgRXRFefmhkOak9DmSvYCCUEuZ64zECcE5RXUZggiNS', 'ROLE_ADMINISTRATOR') ON CONFLICT DO NOTHING; 
COMMIT;

CREATE TABLE IF NOT EXISTS soft_constraints (
  id SERIAL PRIMARY KEY
);

BEGIN;
    with data_soft_constraints(id) as (
      VALUES
      (1),
      (2),
      (3),
      (4)
    )
    INSERT INTO soft_constraints (id) SELECT  d.id from data_soft_constraints d
    WHERE NOT EXISTS (SELECT * FROM soft_constraints);
COMMIT;

CREATE TABLE IF NOT EXISTS prefers (
  id SERIAL PRIMARY KEY,
  soft_c_id INTEGER NOT NULL,
  day VARCHAR NOT NULL,
  num INTEGER NOT NULL,
  CONSTRAINT fk_soft_c  FOREIGN KEY(soft_c_id) REFERENCES soft_constraints(id)
);

BEGIN;
    with data_prefers(soft_c_id, day, num) as (
      VALUES
      (1, 'monday', 1),
      (2, 'monday', 4),
      (3, 'monday', 3),
      (4, 'monday', 1),
      (1, 'tuesday', 2),
      (2, 'tuesday', 2),
      (3, 'tuesday', 2),
      (4, 'tuesday', 1),
      (1, 'wednesday', 3),
      (2, 'wednesday', 2),
      (3, 'wednesday', 5),
      (4, 'wednesday', 3),
      (1, 'thursday', 4),
      (2, 'thursday', 1),
      (3, 'thursday', 4),
      (4, 'thursday', 4),
      (1, 'friday', 5),
      (2, 'friday', 2),
      (3, 'friday', 5),
      (4, 'friday', 1)
    )
    INSERT INTO prefers (soft_c_id, day, num) SELECT  d.soft_c_id, d.day, d.num from data_prefers d
    WHERE NOT EXISTS (SELECT * FROM prefers);
COMMIT;

CREATE TABLE IF NOT EXISTS staffs (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  surname VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  soft_constraints INTEGER NOT NULL,
  faculty INTEGER NOT NULL,
  role VARCHAR NOT NULL,
  CONSTRAINT fk_soft_constraints  FOREIGN KEY (soft_constraints) REFERENCES soft_constraints(id)
);

BEGIN;
    with data_staff(name, surname, email, password, soft_constraints, faculty, role) as (
      VALUES
      ('John', 'Doe', 'j.doe@prof.com', '$2b$12$0KOyov06TutgRXRFefmhkOak9DmSvYCCUEuZ64zECcE5RXUZggiNS', 1, 1, 'ROLE_STAFF'),
      ('Mary', 'Poppins', 'm.poppins@prof.com', '$2b$12$0KOyov06TutgRXRFefmhkOak9DmSvYCCUEuZ64zECcE5RXUZggiNS', 2, 1, 'ROLE_STAFF'),
      ('Kevin', 'Heck', 'k.heck@prof.com', '$2b$12$0KOyov06TutgRXRFefmhkOak9DmSvYCCUEuZ64zECcE5RXUZggiNS', 3, 1, 'ROLE_STAFF'),
      ('Lucy', 'Gray', 'l.gray@prof.com', '$2b$12$0KOyov06TutgRXRFefmhkOak9DmSvYCCUEuZ64zECcE5RXUZggiNS', 4, 1, 'ROLE_STAFF')
    )
    INSERT INTO staffs (name, surname, email, password, soft_constraints, faculty, role) 
    SELECT  d.name, d.surname, d.email, d.password, d.soft_constraints, d.faculty, d.role from data_staff d
    WHERE NOT EXISTS (SELECT * FROM staffs);
COMMIT;

CREATE TABLE IF NOT EXISTS lectures_staffs (
  id SERIAL PRIMARY KEY,
  staff_id INTEGER NOT NULL,
  lecture_id INTEGER NOT NULL,
  CONSTRAINT fk_staff  FOREIGN KEY(staff_id) REFERENCES staffs(id)
);

BEGIN;
    with data_lectures_staffs(staff_id, lecture_id) as (
      VALUES
      (1, 1),
      (1, 2),
      (1, 3),
      (1, 4),
      (2, 1),
      (2, 2),
      (2, 3),
      (2, 4),
      (3, 1),
      (3, 2),
      (3, 3),
      (3, 4),
      (4, 1),
      (4, 2),
      (4, 3),
      (4, 4)
    )
    INSERT INTO lectures_staffs (staff_id, lecture_id) 
    SELECT  d.staff_id, d.lecture_id from data_lectures_staffs d
    WHERE NOT EXISTS (SELECT * FROM lectures_staffs);
COMMIT;
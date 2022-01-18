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

CREATE TABLE IF NOT EXISTS staffs (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  surname VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  soft_constraints INTEGER NOT NULL,
  faculty INTEGER NOT NULL,
  role VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS lectures_staffs (
  id SERIAL PRIMARY KEY,
  staff_id INTEGER NOT NULL,
  lecture_id INTEGER NOT NULL,
  CONSTRAINT fk_staff  FOREIGN KEY(staff_id) REFERENCES staffs(id)
);


CREATE TABLE IF NOT EXISTS soft_constraints (
  id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS prefers (
  id SERIAL PRIMARY KEY,
  soft_c_id INTEGER NOT NULL,
  day VARCHAR NOT NULL,
  num INTEGER NOT NULL,
  CONSTRAINT fk_soft_c  FOREIGN KEY(soft_c_id) REFERENCES soft_constraints(id)
);
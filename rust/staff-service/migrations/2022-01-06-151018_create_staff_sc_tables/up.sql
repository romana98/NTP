-- Your SQL goes here
CREATE TABLE  staffs (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  surname VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  soft_constraints INTEGER NOT NULL,
  faculty INTEGER NOT NULL,
  role VARCHAR NOT NULL
  
);

CREATE TABLE  lectures_staffs (
  id SERIAL PRIMARY KEY,
  staff_id INTEGER NOT NULL,
  lecture_id INTEGER NOT NULL,
  CONSTRAINT fk_staff  FOREIGN KEY(staff_id) REFERENCES staffs(id)
  
);


CREATE TABLE  soft_constraints (
  id SERIAL PRIMARY KEY
);

CREATE TABLE  prefers (
  id SERIAL PRIMARY KEY,
  soft_c_id INTEGER NOT NULL,
  day VARCHAR NOT NULL,
  num INTEGER NOT NULL,
  CONSTRAINT fk_soft_c  FOREIGN KEY(soft_c_id) REFERENCES soft_constraints(id)
);
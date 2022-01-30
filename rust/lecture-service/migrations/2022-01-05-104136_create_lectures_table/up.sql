-- Your SQL goes here
CREATE TABLE IF NOT EXISTS lectures (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  number_of_times INTEGER NOT NULL
);

BEGIN;
    with data_lecture(name, number_of_times) as (
      VALUES
      ('oop1', 1),
      ('ntp', 2),
      ('web', 3),
      ('kts', 4),
      ('nvt', 1),
      ('xml', 2),
      ('alg', 3),
      ('bp', 4),
      ('os', 1),
      ('mreze', 2),
      ('spp', 3),
      ('pp', 4),
      ('isa', 1),
      ('mrs', 2),
      ('pigkut', 3),
      ('usi', 4)
    )
    INSERT INTO lectures (name, number_of_times) SELECT  d.name, d.number_of_times from data_lecture d
    WHERE NOT EXISTS (SELECT * FROM lectures);
COMMIT;
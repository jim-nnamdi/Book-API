-- Your SQL goes here
CREATE TABLE books (
  id INTEGER PRIMARY KEY NOT NULL,
  title VARCHAR NOT NULL,
  author VARCHAR NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);
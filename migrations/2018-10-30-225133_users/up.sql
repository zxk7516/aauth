-- Your SQL goes here

CREATE TABLE users (
  id serial not null primary key,
  name VARCHAR(50) NOT NULL,
  email VARCHAR(100) NOT NULL,
  password VARCHAR(64) NOT NULL, --bcrypt hash
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE invitations (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  expires_at TIMESTAMP NOT NULL
);
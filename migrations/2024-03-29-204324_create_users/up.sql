-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    firstname VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    administrator BOOLEAN NOT NULL DEFAULT FALSE
)
CREATE TABLE events (
    id serial PRIMARY KEY,
    event_id VARCHAR NOT NULL UNIQUE,
    content TEXT NOT NULL
);

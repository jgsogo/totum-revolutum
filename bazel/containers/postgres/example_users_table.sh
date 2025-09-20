#!/usr/bin/env bash
set -euo pipefail

# Create table and insert rows
PGPASSWORD=$POSTGRES_PASSWORD psql --username=$POSTGRES_USER --dbname=$POSTGRES_DB --host=$POSTGRES_HOST --port=$POSTGRES_PORT <<'SQL'
CREATE TABLE users (
    id   SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    age  INT
);

INSERT INTO users (name, age) VALUES
  ('Alice', 30),
  ('Bob',   25),
  ('Carol', 27);

SELECT * FROM users;
SQL

echo "Database $POSTGRES_DB created and populated."

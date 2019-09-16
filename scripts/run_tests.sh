#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if a user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# Check if a password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a password has been set, otherwise default to 'realworld'
DB_NAME="${POSTGRES_DB:=realword}"
# Check if a port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

# Reset DB (in case it was left in a corrupted state)
diesel database reset

# Run tests
cargo test

# Reset DB
diesel database reset

#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if a user has been set
if [[ -z "${POSTGRES_USER}" ]]
then
    echo "POSTGRES_USER has not been set, defaulting to 'postgres'"
    DB_USER=postgres
else
    DB_USER="${POSTGRES_USER}"
fi

# Check if a password has been set
if [[ -z "${POSTGRES_PASSWORD}" ]]
then
    echo "POSTGRES_PASSWORD has not been set, defaulting to 'password'"
    DB_PASSWORD=password
else
    DB_PASSWORD="${POSTGRES_PASSWORD}"
fi

# Check if a host has been set
if [[ -z "${POSTGRES_HOST}" ]]
then
    echo "POSTGRES_HOST has not been set, defaulting to 'localhost'"
    DB_HOST=localhost
else
    DB_HOST="${POSTGRES_HOST}"
fi

# Check if a port has been set
if [[ -z "${POSTGRES_PORT}" ]]
then
    echo "POSTGRES_PORT has not been set, defaulting to '5432'"
    DB_PORT=5432
else
    DB_PORT="${POSTGRES_PORT}"
fi

# Keep pinging until it's ready
until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running - ready to be used!"

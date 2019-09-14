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

# Check if a database name has been set
if [[ -z "${POSTGRES_DB}" ]]
then
    echo "POSTGRES_DB has not been set, defaulting to 'diesel_demo'"
    DB_NAME=diesel_demo
else
    DB_NAME="${POSTGRES_DB}"
fi

# Check if a port has been set
if [[ -z "${POSTGRES_PORT}" ]]
then
    echo "POSTGRES_PORT has not been set, defaulting to '5432'"
    DB_PORT=5432
else
    DB_PORT="${POSTGRES_PORT}"
fi

docker run \
    --name realworld-postgres --rm \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres

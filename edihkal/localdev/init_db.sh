#!/usr/bin/env bash
set -eo pipefail

localdev_dir="$(dirname -- $( readlink -f -- "$0"; ))"

if ! [ -x "$(command -v pg_isready)" ]; then
  echo >&2 "Error: pg_isready is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi


DB_USER=${POSTGRES_USER:=edihkal}
DB_PASSWORD="${POSTGRES_PASSWORD:=changeme}"
DB_NAME="${POSTGRES_DB:=edihkal}"
DB_HOST="${POSTGRES_HOST:=127.0.0.1}"
DB_PORT="${POSTGRES_PORT:=5432}"

# Set to skip container start if edihcal-timescaledb is already running
if [[ -z "${SKIP_STARTUP}" ]]
then
  podman run --name edihkal-timescaledb \
             -p "127.0.0.1:${DB_PORT}:5432" \
             -e POSTGRES_DB=${DB_NAME} \
             -e POSTGRES_PASSWORD=${DB_PASSWORD} \
             -e POSTGRES_USER=${DB_USER} \
             -d \
             timescale/timescaledb:latest-pg14
fi

# Wait until DB is ready
export PGPASSWORD="${DB_PASSWORD}"
until pg_isready -h "${DB_HOST}" -p "${DB_PORT}" -d postgres -U "$DB_USER"; do
  >&2 echo "Postgres is still unavailable. Sleeping..."
  sleep 1
done

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
sqlx database create
sqlx migrate run --source "${localdev_dir}/../migrations"

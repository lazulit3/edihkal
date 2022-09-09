#!/usr/bin/env bash
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

localdev_dir="$(dirname -- $( readlink -f -- "$0"; ))"
source "${localdev_dir}/envs"

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
until psql -h "127.0.0.1" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable. Sleeping..."
  sleep 1
done

sqlx database create
sqlx migrate run --source "${localdev_dir}/../migrations"

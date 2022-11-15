#!/usr/bin/env bash
set -eo pipefail

localdev_dir="$(dirname -- $( readlink -f -- "$0"; ))"

if ! [ -x "$(command -v pg_isready)" ]; then
  echo >&2 "Error: pg_isready is not installed."
  exit 1
fi

# Default to using podman if CONTAINER_TOOL is not specified.
CONTAINER_TOOL="${CONTAINER_TOOL:-podman}"

if [[ -z "${SKIP_STARTUP}" ]] && ! [ -x "$(command -v "$CONTAINER_TOOL")" ]; then
  echo >&2 "Error: Container tool ${CONTAINER_TOOL} is not installed."
  echo >&2 "You may choose which tool should manage the DB container by setting CONTAINER_TOOL."
  exit 1
fi

DB_USER=${POSTGRES_USER:=edihkal}
DB_PASSWORD="${POSTGRES_PASSWORD:=changeme}"
DB_NAME="${POSTGRES_DB:=edihkal}"
DB_HOST="${POSTGRES_HOST:=127.0.0.1}"
DB_PORT="${POSTGRES_PORT:=5432}"

"$CONTAINER_TOOL" run --name edihkal-db \
           -p "127.0.0.1:${DB_PORT}:5432" \
           -e POSTGRES_DB=${DB_NAME} \
           -e POSTGRES_PASSWORD=${DB_PASSWORD} \
           -e POSTGRES_USER=${DB_USER} \
           -d \
           postgres:15-alpine

# Wait until DB is ready
export PGPASSWORD="${DB_PASSWORD}"
until pg_isready -h "${DB_HOST}" -p "${DB_PORT}" -d postgres -U "$DB_USER"; do
  >&2 echo "Postgres is still unavailable. Sleeping..."
  sleep 1
done

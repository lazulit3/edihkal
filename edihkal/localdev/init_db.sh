#!/usr/bin/env bash
set -eo pipefail

if command -v podman &> /dev/null; then
  CONTAINER_TOOL="podman"
elif command -v docker &> /dev/null; then
  CONTAINER_TOOL="docker"
else
  echo "podman or docker must be installed."
  exit 1
fi

DB_USER=${POSTGRES_USER:=edihkal}
DB_PASSWORD="${POSTGRES_PASSWORD:=changeme}"
DB_NAME="${POSTGRES_DB:=edihkal}"
DB_HOST="${POSTGRES_HOST:=127.0.0.1}"
DB_PORT="${POSTGRES_PORT:=5432}"

"$CONTAINER_TOOL" run --name edihkal-db \
           -p "127.0.0.1:${DB_PORT}:5432" \
           -e POSTGRES_DB="$DB_NAME" \
           -e POSTGRES_PASSWORD="$DB_PASSWORD" \
           -e POSTGRES_USER="$DB_USER" \
           -d \
           postgres:15-alpine

# Wait until DB is ready
export PGPASSWORD="${DB_PASSWORD}"
until pg_isready -h "${DB_HOST}" -p "${DB_PORT}" -d postgres -U "$DB_USER"; do
  >&2 echo "Postgres is still unavailable. Sleeping..."
  sleep 1
done
